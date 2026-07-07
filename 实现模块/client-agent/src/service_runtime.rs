use crate::config::{AgentConfig, default_config_path};
use crate::local_log::LocalLog;
use crate::monitor;
use std::env;
use std::error::Error;
use std::io;
use std::process::{Command, Output};

pub const SERVICE_NAME: &str = "WoWClientAgent";
const SERVICE_DISPLAY_NAME: &str = "WoW Client Agent";

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, service_main);

pub fn run_service() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    {
        windows_service::service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
        Ok(())
    }

    #[cfg(not(windows))]
    {
        Err("Windows Service 只支持 Windows".into())
    }
}

pub fn install_service() -> io::Result<String> {
    let exe = env::current_exe()?;
    let exe_text = exe.display().to_string();
    let bin_path = format!("\"{exe_text}\" --service-run");
    let output = run_sc([
        "create",
        SERVICE_NAME,
        "binPath=",
        &bin_path,
        "start=",
        "auto",
        "DisplayName=",
        SERVICE_DISPLAY_NAME,
    ])?;
    ensure_success(output, "安装 Windows Service")?;
    Ok(format!("Windows Service 已安装：{SERVICE_NAME}"))
}

pub fn uninstall_service() -> io::Result<String> {
    let output = run_sc(["delete", SERVICE_NAME])?;
    ensure_success(output, "卸载 Windows Service")?;
    Ok(format!("Windows Service 已卸载：{SERVICE_NAME}"))
}

pub fn start_service() -> io::Result<String> {
    let output = run_sc(["start", SERVICE_NAME])?;
    ensure_success(output, "启动 Windows Service")?;
    Ok(format!("Windows Service 已启动：{SERVICE_NAME}"))
}

pub fn stop_service() -> io::Result<String> {
    let output = run_sc(["stop", SERVICE_NAME])?;
    ensure_success(output, "停止 Windows Service")?;
    Ok(format!("Windows Service 已请求停止：{SERVICE_NAME}"))
}

pub fn service_status() -> io::Result<String> {
    let output = run_sc(["query", SERVICE_NAME])?;
    if !output.status.success() {
        return Ok(format!("Windows Service 未安装或不可查询：{SERVICE_NAME}"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn run_sc<const N: usize>(args: [&str; N]) -> io::Result<Output> {
    Command::new("sc.exe").args(args).output()
}

fn ensure_success(output: Output, action: &str) -> io::Result<()> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detail = if !stderr.is_empty() { stderr } else { stdout };
    Err(io::Error::other(format!(
        "{action}失败：{}",
        if detail.is_empty() {
            "sc.exe 未返回错误详情"
        } else {
            &detail
        }
    )))
}

#[cfg(windows)]
fn service_main(_arguments: Vec<std::ffi::OsString>) {
    if let Err(error) = run_service_inner() {
        let _ = LocalLog::default().append_event(&format!("Windows Service 运行失败：{error}"));
    }
}

#[cfg(windows)]
fn run_service_inner() -> Result<(), Box<dyn Error>> {
    use std::sync::mpsc;
    use windows_service::service::{ServiceControl, ServiceControlAccept, ServiceState};
    use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop | ServiceControl::Shutdown => {
                let _ = shutdown_tx.send(());
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
    set_service_status(
        &status_handle,
        ServiceState::StartPending,
        ServiceControlAccept::empty(),
    )?;

    let config = AgentConfig::load_from_path(default_config_path())?;
    set_service_status(
        &status_handle,
        ServiceState::Running,
        ServiceControlAccept::STOP,
    )?;
    let monitor_result = monitor::run_monitor_until_shutdown(config, Some(shutdown_rx));

    set_service_status(
        &status_handle,
        ServiceState::StopPending,
        ServiceControlAccept::empty(),
    )?;
    if let Err(error) = monitor_result {
        let _ = LocalLog::default().append_event(&format!("Service monitor 异常退出：{error}"));
    }
    set_service_status(
        &status_handle,
        ServiceState::Stopped,
        ServiceControlAccept::empty(),
    )?;
    Ok(())
}

#[cfg(windows)]
fn set_service_status(
    handle: &windows_service::service_control_handler::ServiceStatusHandle,
    state: windows_service::service::ServiceState,
    controls: windows_service::service::ServiceControlAccept,
) -> windows_service::Result<()> {
    use std::time::Duration;
    use windows_service::service::{ServiceExitCode, ServiceStatus, ServiceType};

    // Service 状态只描述 monitor 生命周期，不暴露脚本配置或本机私有路径。
    // 输入：SCM 控制事件和 monitor 停止信号。
    // 输出：Windows Service Control Manager 可识别的状态。
    // 边界：托盘 UI 与 Service 分开运行，避免 Session 0 交互限制。
    handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: state,
        controls_accepted: controls,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_names_are_stable() {
        assert_eq!(SERVICE_NAME, "WoWClientAgent");
        assert_eq!(SERVICE_DISPLAY_NAME, "WoW Client Agent");
    }
}
