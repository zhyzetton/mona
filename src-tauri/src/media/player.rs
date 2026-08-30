use std::process::Command;

pub fn open_with_system_default(path: &str) -> std::io::Result<std::process::Child> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "start", "", path]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()
    } else {
        Command::new("xdg-open").arg(path).spawn()
    }
}