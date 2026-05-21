use crate::cmds::switch::switch_account;
use crate::common::{get_accounts, get_steam_path, load_tags};
use crate::os::SteamPlatform;
use std::io::{self, Write};

pub fn run(platform: &dyn SteamPlatform) -> Result<(), String> {
    let steam_path = get_steam_path(platform)?;
    let steam_exe = platform.detect_steam_exe(&steam_path);
    let accounts = get_accounts(&steam_path);
    if accounts.is_empty() {
        return Err(
            "no login profiles found in loginusers.vdf.\nlogin to steam at least once first"
                .to_string(),
        );
    }

    let active_user = platform
        .get_active_user(&steam_path)
        .unwrap_or_else(|_| "None".to_string());
    let running = platform.is_steam_running();

    println!("steam path: {}", steam_path.display());
    println!(
        "status:     {}",
        if running { "running" } else { "stopped" }
    );
    println!("active:     {}", active_user);
    println!();

    let (tags, _) = load_tags();
    println!("available accounts:");
    for (i, acc) in accounts.iter().enumerate() {
        let mut status = Vec::with_capacity(3);
        if acc.username.eq_ignore_ascii_case(&active_user) {
            status.push("active".to_string());
        }
        if acc.most_recent {
            status.push("last used".to_string());
        }
        for (tag, steam_id) in &tags {
            if steam_id == &acc.steam_id {
                status.push(format!("tag: {}", tag));
            }
        }
        let status_str = if status.is_empty() {
            String::new()
        } else {
            format!(" [{}]", status.join(", "))
        };
        println!(
            "  {}. {} ({}){}",
            i + 1,
            acc.display_name,
            acc.username,
            status_str
        );
    }

    print!("\nselect account number to switch to (or press enter to cancel): ");
    io::stdout().flush().ok();

    let mut input = String::with_capacity(2);
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;
    let input = input.trim();
    if input.is_empty() {
        println!("canceled.");
        return Ok(());
    }

    if let Ok(idx) = input.parse::<usize>()
        && (idx > 0 && idx <= accounts.len())
    {
        let target = &accounts[idx - 1];
        if target.username.eq_ignore_ascii_case(&active_user) && running {
            println!(
                "steam is already running with {} ({})",
                target.display_name, target.username
            );
            return Ok(());
        }
        switch_account(platform, &steam_path, &steam_exe, target)?;
        return Ok(());
    }

    Err("invalid selection".to_string())
}
