#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    wow_launcher::run_or_log(wow_launcher::LaunchTarget::Client);
}
