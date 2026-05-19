use super::SteamPlatform;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct WindowsPlatform;

impl WindowsPlatform {
    fn get_registry_value(&self, key: &str, value_name: &str) -> Option<String> {
        let output = Command::new("reg")
            .args(["query", key, "/v", value_name])
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.to_lowercase().contains(&value_name.to_lowercase()) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    if parts[1].eq_ignore_ascii_case("REG_SZ") {
                        let idx = line.find("REG_SZ").unwrap() + 6;
                        return Some(line[idx..].trim().to_string());
                    } else if parts[1].eq_ignore_ascii_case("REG_DWORD") {
                        let hex_str = parts[2].trim_start_matches("0x");
                        if let Ok(val) = u32::from_str_radix(hex_str, 16) {
                            return Some(val.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    fn set_registry_value_sz(
        &self,
        key: &str,
        value_name: &str,
        value: &str,
    ) -> Result<(), String> {
        let output = Command::new("reg")
            .args([
                "add", key, "/v", value_name, "/t", "REG_SZ", "/d", value, "/f",
            ])
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        Ok(())
    }

    fn set_registry_value_dword(
        &self,
        key: &str,
        value_name: &str,
        value: u32,
    ) -> Result<(), String> {
        let output = Command::new("reg")
            .args([
                "add",
                key,
                "/v",
                value_name,
                "/t",
                "REG_DWORD",
                "/d",
                &value.to_string(),
                "/f",
            ])
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        Ok(())
    }
}

impl SteamPlatform for WindowsPlatform {
    fn detect_steam_path(&self) -> Option<PathBuf> {
        if let Some(steam_path_str) =
            self.get_registry_value(r"HKCU\Software\Valve\Steam", "SteamPath")
            && let path = PathBuf::from(&steam_path_str)
            && path.exists()
        {
            return Some(path);
        }

        let standard_paths = [
            r"C:\Program Files (x86)\Steam",
            r"C:\Program Files\Steam",
            r"D:\Steam",
            r"D:\Games\Steam",
            r"E:\Steam",
        ];
        for &sp in &standard_paths {
            let path = PathBuf::from(sp);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    fn detect_steam_exe(&self, steam_path: &Path) -> PathBuf {
        if let Some(steam_exe_str) =
            self.get_registry_value(r"HKCU\Software\Valve\Steam", "SteamExe")
            && let exe = PathBuf::from(steam_exe_str)
            && exe.exists()
        {
            return exe;
        }
        steam_path.join("steam.exe")
    }

    fn get_active_user(&self, _steam_path: &Path) -> Result<String, String> {
        self.get_registry_value(r"HKCU\Software\Valve\Steam", "AutoLoginUser")
            .ok_or_else(|| "none".to_string())
    }

    fn set_active_user(&self, _steam_path: &Path, username: &str) -> Result<(), String> {
        self.set_registry_value_sz(r"HKCU\Software\Valve\Steam", "AutoLoginUser", username)?;
        self.set_registry_value_dword(r"HKCU\Software\Valve\Steam", "RememberPassword", 1)?;
        Ok(())
    }

    fn is_steam_running(&self) -> bool {
        let output = Command::new("tasklist")
            .args(["/fi", "imagename eq steam.exe"])
            .output();
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
            return stdout.contains("steam.exe");
        }
        false
    }

    fn kill_steam(&self) -> Result<(), String> {
        Command::new("taskkill")
            .args(["/f", "/im", "steam.exe"])
            .output()
            .map_err(|e| format!("failed to execute taskkill for steam.exe: {e}"))?;
        Command::new("taskkill")
            .args(["/f", "/im", "steamwebhelper.exe"])
            .output()
            .map_err(|e| format!("failed to execute taskkill for steamwebhelper.exe: {e}"))?;
        Ok(())
    }

    fn start_steam(
        &self,
        _steam_path: &Path,
        steam_exe: &Path,
        silent: bool,
    ) -> Result<(), String> {
        let mut cmd = Command::new(steam_exe);
        if silent {
            cmd.args(["-silent"]);
        }
        cmd.spawn().map_err(|e| e.to_string())?;
        Ok(())
    }
}
