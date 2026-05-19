use super::SteamPlatform;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct UnixPlatform;

impl UnixPlatform {
    fn get_unix_registry_path(&self, steam_path: &Path) -> Option<PathBuf> {
        let p1 = steam_path.join("registry.vdf");
        if p1.exists() {
            return Some(p1);
        }
        if let Some(parent) = steam_path.parent()
            && let p2 = parent.join("registry.vdf")
            && p2.exists()
        {
            return Some(p2);
        }
        None
    }

    fn get_registry_vdf_value(&self, registry_path: &Path, value_name: &str) -> Option<String> {
        let content = fs::read_to_string(registry_path).ok()?;
        let parsed = vdf::parse(&content).ok()?;
        let registry = parsed.get("Registry")?;
        let hkcu = registry.get("HKCU")?;
        let software = hkcu.get("Software")?;
        let valve = software.get("Valve")?;
        let steam = valve.get("Steam")?;
        let val = steam.get(value_name)?;
        val.get_str().map(|s| s.to_string())
    }

    fn set_registry_vdf_value(
        &self,
        registry_path: &Path,
        value_name: &str,
        new_value: &str,
    ) -> Result<(), String> {
        let content = fs::read_to_string(registry_path).map_err(|e| e.to_string())?;
        let mut parsed = vdf::parse(&content)?;

        {
            let registry = parsed
                .entry("Registry".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(BTreeMap::new()))
                .get_obj_mut()
                .ok_or("invalid registry.vdf root structure")?;

            let hkcu = registry
                .entry("HKCU".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(BTreeMap::new()))
                .get_obj_mut()
                .ok_or("invalid HKCU")?;

            let software = hkcu
                .entry("Software".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(BTreeMap::new()))
                .get_obj_mut()
                .ok_or("invalid Software")?;

            let valve = software
                .entry("Valve".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(BTreeMap::new()))
                .get_obj_mut()
                .ok_or("invalid Valve")?;

            let steam = valve
                .entry("Steam".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(BTreeMap::new()))
                .get_obj_mut()
                .ok_or("invalid Steam")?;

            steam.insert(
                value_name.to_string(),
                vdf::VdfValue::Str(new_value.to_string()),
            );
        }

        let stringified = vdf::stringify(&parsed);
        fs::write(registry_path, stringified).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl SteamPlatform for UnixPlatform {
    fn detect_steam_path(&self) -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let home_path = Path::new(&home);

        let paths = if cfg!(target_os = "macos") {
            vec![home_path.join("Library/Application Support/Steam")]
        } else {
            vec![
                home_path.join(".steam/steam"),
                home_path.join(".steam/root"),
                home_path.join(".local/share/Steam"),
                // flatpak
                home_path.join(".var/app/com.valvesoftware.Steam/.steam/steam"),
            ]
        };

        for p in paths {
            if p.exists() && p.join("config/loginusers.vdf").exists() {
                return Some(p);
            }
        }
        None
    }

    fn detect_steam_exe(&self, steam_path: &Path) -> PathBuf {
        if cfg!(target_os = "macos") {
            steam_path.join("Steam.app/Contents/MacOS/steam_osx")
        } else {
            steam_path.join("steam")
        }
    }

    fn get_active_user(&self, steam_path: &Path) -> Result<String, String> {
        let reg_path = self
            .get_unix_registry_path(steam_path)
            .ok_or_else(|| "could not locate registry.vdf".to_string())?;
        self.get_registry_vdf_value(&reg_path, "AutoLoginUser")
            .ok_or_else(|| "none".to_string())
    }

    fn set_active_user(&self, steam_path: &Path, username: &str) -> Result<(), String> {
        let reg_path = self
            .get_unix_registry_path(steam_path)
            .ok_or_else(|| "Could not locate registry.vdf".to_string())?;
        self.set_registry_vdf_value(&reg_path, "AutoLoginUser", username)?;
        self.set_registry_vdf_value(&reg_path, "RememberPassword", "1")?;
        Ok(())
    }

    fn is_steam_running(&self) -> bool {
        let output = Command::new("pgrep").args(&["-x", "steam"]).output();
        if let Ok(out) = output {
            return out.status.success();
        }
        false
    }

    fn kill_steam(&self) -> Result<(), String> {
        Command::new("pkill")
            .args(&["-f", "steam"])
            .output()
            .map_err(|e| format!("failed to execute pkill: {e}"))?;
        Ok(())
    }

    fn start_steam(
        &self,
        _steam_path: &Path,
        steam_exe: &Path,
        silent: bool,
    ) -> Result<(), String> {
        let mut cmd = Command::new(steam_exe);
        if !steam_exe.exists() {
            cmd = Command::new("steam");
        }
        if silent {
            cmd.args(["-silent"]);
        }
        cmd.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
