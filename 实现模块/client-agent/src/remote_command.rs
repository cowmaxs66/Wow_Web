use crate::local_log::LocalLog;
use std::error::Error;

pub fn execute_remote_command(command_type: &str) -> Result<String, Box<dyn Error>> {
    match command_type {
        "startup.status" => Ok(crate::startup::startup_status()?.summary()),
        "startup.enable" => Ok(crate::startup::enable_startup()?.summary()),
        "startup.disable" => Ok(crate::startup::disable_startup()?.summary()),
        "service.status" => Ok(crate::service_runtime::service_status()?),
        "service.install" => Ok(crate::service_runtime::install_service()?),
        "service.start" => Ok(crate::service_runtime::start_service()?),
        "service.stop" => Ok(crate::service_runtime::stop_service()?),
        "update.check" => Ok(crate::updater::check_update()?),
        "update.download" => Ok(crate::updater::download_update()?),
        "settings.open" => {
            crate::settings_window::open_settings_window()?;
            Ok("已请求打开设置窗口".to_string())
        }
        "log.open" => {
            LocalLog::default().open_event_log()?;
            Ok("已请求打开日志窗口".to_string())
        }
        "tray.open" => {
            crate::tray::run_tray()?;
            Ok("已请求打开托盘".to_string())
        }
        unknown => Err(format!("不支持的远程命令：{unknown}").into()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unsupported_remote_command_is_rejected() {
        let error = super::execute_remote_command("shell.exec").expect_err("must reject shell");
        assert!(error.to_string().contains("不支持"));
    }
}
