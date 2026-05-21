use crate::common::{get_accounts, get_steam_path, load_tags, save_tags};
use crate::os::SteamPlatform;

pub fn run(
    platform: &dyn SteamPlatform,
    tag_name: &str,
    account_ident: &str,
) -> Result<(), String> {
    let tag_name = tag_name.trim().to_lowercase();
    if tag_name.is_empty() {
        return Err("tag name cannot be empty".to_string());
    }
    if !tag_name.chars().all(|c| c.is_alphanumeric()) {
        return Err("tag name must be alphanumeric".to_string());
    }

    const RESERVED: &[&str] = &[
        "stop",
        "path",
        "skip-offline",
        "start",
        "tag",
        "untag",
        "tags",
        "help",
    ];

    if RESERVED.contains(&tag_name.as_str()) {
        return Err(format!(
            "'{}' is a reserved command and cannot be used as a tag",
            tag_name
        ));
    }

    let steam_path = get_steam_path(platform)?;
    let accounts = get_accounts(&steam_path);
    if accounts.is_empty() {
        return Err("no login profiles found. login to steam at least once first".to_string());
    }

    let mut found_account = None;

    if let Ok(idx) = account_ident.parse::<usize>()
        && idx > 0
        && idx <= accounts.len()
    {
        found_account = Some(&accounts[idx - 1]);
    }

    if found_account.is_none() {
        for acc in &accounts {
            if acc.username.eq_ignore_ascii_case(account_ident)
                || acc.steam_id == account_ident
                || acc.display_name.eq_ignore_ascii_case(account_ident)
            {
                found_account = Some(acc);
                break;
            }
        }
    }

    let target =
        found_account.ok_or_else(|| format!("account not found matching '{}'", account_ident))?;

    let (mut tags, cap) = load_tags();
    tags.insert(tag_name.clone(), target.steam_id.clone());
    let new_cap = cap + tag_name.len() + target.steam_id.len() + 2;
    save_tags(&tags, new_cap)?;

    println!(
        "tagged {} ({}) as '{}'",
        target.display_name, target.username, tag_name
    );
    Ok(())
}
