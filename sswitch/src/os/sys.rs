#[cfg(target_os = "windows")]
pub use super::windows::WindowsPlatform as CurrentPlatform;

#[cfg(not(target_os = "windows"))]
pub use super::unix::UnixPlatform as CurrentPlatform;
