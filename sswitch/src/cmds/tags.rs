use crate::common::{get_accounts, get_steam_path, load_tags};
use crate::os::SteamPlatform;

pub fn run(platform: &dyn SteamPlatform) -> Result<(), String> {
    let (tags, _) = load_tags();
    if tags.is_empty() {
        println!("no tags configured.");
        return Ok(());
    }

    let steam_path = get_steam_path(platform)?;
    let accounts = get_accounts(&steam_path);

    println!("configured tags:");
    for (tag, steam_id) in &tags {
        let acc_str = if let Some(acc) = accounts.iter().find(|a| &a.steam_id == steam_id) {
            format!("{} ({})", acc.display_name, acc.username)
        } else {
            format!("SteamID: {}", steam_id)
        };
        println!("  {} -> {}", tag, acc_str);
    }
    Ok(())
}
