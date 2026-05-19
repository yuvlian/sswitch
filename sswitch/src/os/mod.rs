use std::path::{Path, PathBuf};

pub trait SteamPlatform {
    fn detect_steam_path(&self) -> Option<PathBuf>;
    fn detect_steam_exe(&self, steam_path: &Path) -> PathBuf;
    fn get_active_user(&self, steam_path: &Path) -> Result<String, String>;
    fn set_active_user(&self, steam_path: &Path, username: &str) -> Result<(), String>;
    fn is_steam_running(&self) -> bool;
    fn kill_steam(&self) -> Result<(), String>;
    fn start_steam(&self, steam_path: &Path, steam_exe: &Path, silent: bool) -> Result<(), String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "windows"))]
pub mod unix;

pub mod sys;
