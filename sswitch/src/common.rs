use crate::os::SteamPlatform;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Account {
    pub steam_id: String,
    pub username: String,
    pub display_name: String,
    pub most_recent: bool,
    pub timestamp: u64,
}

pub fn get_accounts(steam_path: &Path) -> Vec<Account> {
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if !loginusers_path.exists() {
        return Vec::new();
    }

    let Ok(content) = fs::read_to_string(&loginusers_path) else {
        return Vec::new();
    };

    let Ok(parsed) = vdf::parse(&content) else {
        return Vec::new();
    };

    let Some(users_val) = parsed.get("users") else {
        return Vec::new();
    };

    let Some(users_map) = users_val.get_obj() else {
        return Vec::new();
    };

    let mut accounts: Vec<Account> = users_map
        .iter()
        .filter_map(|(steam_id, user_data_val)| {
            let user_data = user_data_val.get_obj()?;
            let username = user_data
                .get("AccountName")
                .and_then(|v| v.get_str())
                .unwrap_or("Unknown")
                .to_string();

            let display_name = user_data
                .get("PersonaName")
                .and_then(|v| v.get_str())
                .unwrap_or(&username)
                .to_string();

            let most_recent = user_data
                .get("MostRecent")
                .and_then(|v| v.get_str())
                .map(|s| s == "1" || s == "true")
                .unwrap_or(false);

            let timestamp = user_data
                .get("Timestamp")
                .and_then(|v| v.get_str())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);

            Some(Account {
                steam_id: steam_id.clone(),
                username,
                display_name,
                most_recent,
                timestamp,
            })
        })
        .collect();

    accounts.sort_by_key(|b| std::cmp::Reverse(b.timestamp));
    accounts
}

pub fn get_steam_path(platform: &dyn SteamPlatform) -> Result<PathBuf, String> {
    let custom_path = fs::read_to_string("custom_steam_path")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    match custom_path {
        Some(ref cp) => {
            let p = PathBuf::from(cp);
            if !p.exists() {
                return Err(format!("custom steam path does not exist: {}", cp));
            }
            Ok(p)
        }
        None => platform.detect_steam_path().ok_or_else(|| {
            "steam path not found automatically.\nspecify it using: sswitch path <path>".to_string()
        }),
    }
}

pub fn load_tags() -> (HashMap<String, String>, usize) {
    if let Ok(content) = fs::read_to_string("tags.txt") {
        let len = content.len();
        let tags = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .filter_map(|line| line.split_once('='))
            .map(|(tag, steam_id)| (tag.trim().to_lowercase(), steam_id.trim().to_string()))
            .collect();
        (tags, len)
    } else {
        (HashMap::new(), 0)
    }
}

pub fn save_tags(tags: &HashMap<String, String>, capacity: usize) -> Result<(), String> {
    let mut content = String::with_capacity(capacity);
    for (tag, steam_id) in tags {
        let _ = writeln!(content, "{tag}={steam_id}");
    }
    fs::write("tags.txt", content).map_err(|e| e.to_string())
}

pub fn simple_account_list(platform: &dyn SteamPlatform) -> String {
    let Ok(steam_path) = get_steam_path(platform) else {
        return String::new();
    };

    let accounts = get_accounts(&steam_path);
    if accounts.is_empty() {
        return String::new();
    }

    let active_user = platform
        .get_active_user(&steam_path)
        .unwrap_or_else(|_| "None".to_string());

    let (tags, _) = load_tags();

    let estimated_per_account = 64;
    let mut out = String::with_capacity(22 + accounts.len() * estimated_per_account);

    out.push_str("\navailable accounts:\n");

    for (i, acc) in accounts.iter().enumerate() {
        let mut statuses = [""; 2];
        let mut status_count = 0;

        if acc.username.eq_ignore_ascii_case(&active_user) {
            statuses[status_count] = "active";
            status_count += 1;
        }
        if acc.most_recent {
            statuses[status_count] = "last used";
            status_count += 1;
        }

        let tag = tags
            .iter()
            .find(|(_, steam_id)| *steam_id == &acc.steam_id)
            .map(|(tag, _)| tag.as_str());

        let _ = write!(out, "  {}. {} ({})", i + 1, acc.display_name, acc.username);

        if status_count != 0 || tag.is_some() {
            out.push_str(" [");
            let mut first = true;
            for status in statuses.iter().take(status_count) {
                if !first {
                    out.push_str(", ");
                }
                first = false;
                out.push_str(status);
            }

            if let Some(tag) = tag {
                if !first {
                    out.push_str(", ");
                }
                out.push_str("tag: ");
                out.push_str(tag);
            }
            out.push(']');
        }
        out.push('\n');
    }
    out
}

pub fn account_help_text(platform: &dyn SteamPlatform) -> String {
    let accounts = simple_account_list(platform);
    let mut out = String::with_capacity(205 + accounts.len());
    out.push_str("where <acc> can be:\n");
    out.push_str("  - the number of the account in the list below (e.g. 1)\n");
    out.push_str("  - the login username (e.g. gabelogannewell)\n");
    out.push_str("  - the display name (e.g. Gabe Newell)\n");
    out.push_str("  - the steam ID (e.g. 76561197960287930)\n");
    out.push_str(&accounts);
    out
}
