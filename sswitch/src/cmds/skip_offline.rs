use crate::common::get_steam_path;
use crate::os::SteamPlatform;
use std::fs;

pub fn run(platform: &dyn SteamPlatform) -> Result<(), String> {
    let steam_path = get_steam_path(platform)?;
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if !loginusers_path.exists() {
        return Err("loginusers.vdf not found".to_string());
    }

    let content = fs::read_to_string(&loginusers_path).map_err(|e| e.to_string())?;
    let mut parsed = vdf::parse(&content)?;

    if let Some(users_val) = parsed.get_mut("users")
        && let Some(users_map) = users_val.get_obj_mut()
    {
        let backup_path = loginusers_path.with_extension("vdf_last");
        fs::copy(&loginusers_path, &backup_path)
            .map_err(|e| format!("failed to backup loginusers.vdf: {e}"))?;

        for user_data_val in users_map.values_mut() {
            if let Some(user_data) = user_data_val.get_obj_mut() {
                user_data.insert(
                    "SkipOfflineModeWarning".to_string(),
                    vdf::VdfValue::Str("1".to_string()),
                );
            }
        }
    }

    let stringified = vdf::stringify(&parsed);
    fs::write(&loginusers_path, stringified).map_err(|e| e.to_string())?;

    println!("enabled SkipOfflineModeWarning for all accounts successfully");
    Ok(())
}
