use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchTarget {
    Server,
    Client,
    Installer,
    Uninstaller,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchPlan {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub working_dir: PathBuf,
}

pub fn run(target: LaunchTarget) -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "无法取得 launcher 所在目录"))?;
    let plan = build_plan(target, exe_dir)?;
    spawn_hidden(&plan)
}

pub fn run_or_log(target: LaunchTarget) {
    if let Err(error) = run(target) {
        let _ = write_launcher_error(target, &error);
    }
}

pub fn build_plan(target: LaunchTarget, launcher_dir: &Path) -> io::Result<LaunchPlan> {
    match target {
        LaunchTarget::Server => core_plan(
            launcher_dir,
            "management-server-core.exe",
            &["--tray"],
            "Management Server 核心程序",
        ),
        LaunchTarget::Client => core_plan(
            launcher_dir,
            "client-agent-core.exe",
            &["--tray"],
            "Client Agent 核心程序",
        ),
        LaunchTarget::Installer => {
            script_plan(launcher_dir, "install-current-user.ps1", "当前用户安装脚本")
        }
        LaunchTarget::Uninstaller => script_plan(
            launcher_dir,
            "uninstall-current-user.ps1",
            "当前用户卸载脚本",
        ),
    }
}

fn core_plan(
    launcher_dir: &Path,
    core_name: &str,
    args: &[&str],
    description: &str,
) -> io::Result<LaunchPlan> {
    let program = resolve_existing_path(
        &[
            launcher_dir.join("bin").join(core_name),
            launcher_dir.join(core_name),
        ],
        description,
    )?;

    // 核心 exe 放在 bin 下时，工作目录必须回到发布包根目录。
    // 输入：launcher 所在目录和 core exe 路径。
    // 输出：核心进程启动目录。
    // 边界：配置、脚本和 DmBridge 都以包根目录相对路径解析。
    let working_dir = package_root_from_core(&program, launcher_dir);

    Ok(LaunchPlan {
        program,
        args: args.iter().map(|arg| (*arg).to_string()).collect(),
        working_dir,
    })
}

fn script_plan(
    launcher_dir: &Path,
    script_name: &str,
    description: &str,
) -> io::Result<LaunchPlan> {
    let script = resolve_existing_path(
        &[
            launcher_dir.join("installer").join(script_name),
            launcher_dir.join(script_name),
        ],
        description,
    )?;

    Ok(LaunchPlan {
        program: powershell_exe(),
        args: vec![
            "-NoProfile".to_string(),
            "-ExecutionPolicy".to_string(),
            "Bypass".to_string(),
            "-File".to_string(),
            script.display().to_string(),
            "-ShowMessage".to_string(),
        ],
        working_dir: launcher_dir.to_path_buf(),
    })
}

fn resolve_existing_path(candidates: &[PathBuf], description: &str) -> io::Result<PathBuf> {
    candidates
        .iter()
        .find(|candidate| candidate.exists())
        .cloned()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("找不到{description}")))
}

fn package_root_from_core(program: &Path, launcher_dir: &Path) -> PathBuf {
    let parent = program.parent().unwrap_or(launcher_dir);
    if parent
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("bin"))
    {
        parent.parent().unwrap_or(launcher_dir).to_path_buf()
    } else {
        launcher_dir.to_path_buf()
    }
}

fn powershell_exe() -> PathBuf {
    PathBuf::from("powershell.exe")
}

fn spawn_hidden(plan: &LaunchPlan) -> io::Result<()> {
    let mut command = Command::new(&plan.program);
    command.args(&plan.args).current_dir(&plan.working_dir);

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.spawn().map(|_| ())
}

fn write_launcher_error(target: LaunchTarget, error: &io::Error) -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "无法取得 launcher 所在目录"))?;
    let log_dir = exe_dir.join("logs");
    fs::create_dir_all(&log_dir)?;

    // GUI 子系统没有控制台，启动失败必须落盘，否则用户双击后没有任何反馈。
    // 输入：启动目标和 io 错误。
    // 输出：logs/launcher-error.log 追加记录。
    // 边界：日志只记录本地路径和错误文本，不写入账号、授权和脚本内容。
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("launcher-error.log"))
        .and_then(|mut file| {
            use std::io::Write;
            writeln!(file, "{target:?}: {error}")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn server_launcher_starts_tray_core() {
        let root = unique_temp_root("server");
        let bin = root.join("bin");
        fs::create_dir_all(&bin).expect("bin must be created");
        fs::write(bin.join("management-server-core.exe"), b"").expect("core must be created");

        let plan = build_plan(LaunchTarget::Server, &root).expect("server plan must build");

        assert_eq!(plan.program, bin.join("management-server-core.exe"));
        assert_eq!(plan.args, vec!["--tray".to_string()]);
        assert_eq!(plan.working_dir, root);
    }

    #[test]
    fn client_launcher_starts_tray_core() {
        let root = unique_temp_root("client");
        let bin = root.join("bin");
        fs::create_dir_all(&bin).expect("bin must be created");
        fs::write(bin.join("client-agent-core.exe"), b"").expect("core must be created");

        let plan = build_plan(LaunchTarget::Client, &root).expect("client plan must build");

        assert_eq!(plan.program, bin.join("client-agent-core.exe"));
        assert_eq!(plan.args, vec!["--tray".to_string()]);
        assert_eq!(plan.working_dir, root);
    }

    #[test]
    fn installer_launcher_calls_packaged_script() {
        let root = unique_temp_root("installer");
        let installer = root.join("installer");
        fs::create_dir_all(&installer).expect("installer dir must be created");
        fs::write(installer.join("install-current-user.ps1"), b"").expect("script must exist");

        let plan = build_plan(LaunchTarget::Installer, &root).expect("installer plan must build");

        assert_eq!(plan.program, PathBuf::from("powershell.exe"));
        assert!(
            plan.args
                .iter()
                .any(|arg| arg.ends_with("install-current-user.ps1"))
        );
        assert_eq!(plan.working_dir, root);
    }

    fn unique_temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("wow-launcher-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }
}
