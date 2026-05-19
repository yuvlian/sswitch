use crate::common::{Account, get_accounts, get_steam_path, load_tags};
use crate::os::SteamPlatform;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn switch_account(
    platform: &dyn SteamPlatform,
    steam_path: &Path,
    steam_exe: &Path,
    target_account: &Account,
) -> Result<(), String> {
    let running = platform.is_steam_running();
    if running {
        println!("steam is running. closing it...");
        platform.kill_steam()?;
        thread::sleep(Duration::from_millis(1500));
    }

    println!("setting auto-login to '{}'", target_account.username);
    platform.set_active_user(steam_path, &target_account.username)?;

    println!("configuring loginusers.vdf...");
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if loginusers_path.exists() {
        let content = fs::read_to_string(&loginusers_path).map_err(|e| e.to_string())?;
        let mut parsed = vdf::parse(&content)?;

        if let Some(users_val) = parsed.get_mut("users")
            && let Some(users_map) = users_val.get_obj_mut()
        {
            let backup_path = loginusers_path.with_extension("vdf_last");
            fs::copy(&loginusers_path, &backup_path)
                .map_err(|e| format!("failed to backup loginusers.vdf: {e}"))?;

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            for (id, user_data_val) in users_map.iter_mut() {
                if id == &target_account.steam_id
                    && let Some(user_data) = user_data_val.get_obj_mut()
                {
                    user_data.insert(
                        "MostRecent".to_string(),
                        vdf::VdfValue::Str("1".to_string()),
                    );
                    user_data.insert(
                        "RememberPassword".to_string(),
                        vdf::VdfValue::Str("1".to_string()),
                    );
                    user_data.insert(
                        "AllowAutoLogin".to_string(),
                        vdf::VdfValue::Str("1".to_string()),
                    );
                    user_data.insert(
                        "Timestamp".to_string(),
                        vdf::VdfValue::Str(current_time.to_string()),
                    );
                } else if let Some(user_data) = user_data_val.get_obj_mut() {
                    user_data.insert(
                        "MostRecent".to_string(),
                        vdf::VdfValue::Str("0".to_string()),
                    );
                    user_data.insert(
                        "AllowAutoLogin".to_string(),
                        vdf::VdfValue::Str("0".to_string()),
                    );
                }
            }
        }

        let stringified = vdf::stringify(&parsed);
        fs::write(&loginusers_path, stringified).map_err(|e| e.to_string())?;
    }

    println!("configuring config.vdf...");
    let config_path = steam_path.join("config").join("config.vdf");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        let mut parsed = vdf::parse(&content)?;

        if let Some(store_val) = parsed.get_mut("InstallConfigStore")
            && let Some(store_map) = store_val.get_obj_mut()
        {
            let backup_path = config_path.with_extension("vdf_last");
            fs::copy(&config_path, &backup_path)
                .map_err(|e| format!("failed to backup config.vdf: {e}"))?;

            let web_storage = store_map
                .entry("WebStorage".to_string())
                .or_insert_with(|| vdf::VdfValue::Obj(std::collections::BTreeMap::new()));

            if let Some(web_storage_map) = web_storage.get_obj_mut() {
                let auth = web_storage_map
                    .entry("Auth".to_string())
                    .or_insert_with(|| vdf::VdfValue::Obj(std::collections::BTreeMap::new()));

                if let Some(auth_map) = auth.get_obj_mut() {
                    auth_map.insert(
                        "AlwaysShowUserChooser".to_string(),
                        vdf::VdfValue::Str("0".to_string()),
                    );
                }
            }
        }

        let stringified = vdf::stringify(&parsed);
        fs::write(&config_path, stringified).map_err(|e| e.to_string())?;
    }

    println!(
        "switched successfully to {} ({})",
        target_account.display_name, target_account.username
    );

    println!("launching steam...");
    platform.start_steam(steam_path, steam_exe, false)?;
    println!("steam client started");

    Ok(())
}

pub fn handle_tag_switch(platform: &dyn SteamPlatform, tag_name: &str) -> Result<(), String> {
    let (tags, _) = load_tags();
    let steam_id = tags.get(&tag_name.to_lowercase()).ok_or_else(|| {
        format!(
            "no tag or command matches '{}'. type 'sswitch help' to see usage",
            tag_name
        )
    })?;

    let steam_path = get_steam_path(platform)?;
    let steam_exe = platform.detect_steam_exe(&steam_path);
    let accounts = get_accounts(&steam_path);
    let target = accounts
        .iter()
        .find(|a| &a.steam_id == steam_id)
        .ok_or_else(|| {
            format!(
                "account with SteamID {} associated with tag '{}' is not found in loginusers.vdf",
                steam_id, tag_name
            )
        })?;

    switch_account(platform, &steam_path, &steam_exe, target)
}
