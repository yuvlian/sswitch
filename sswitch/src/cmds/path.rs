use std::fs;

pub fn run(path: &str) -> Result<(), String> {
    fs::write("custom_steam_path", path)
        .map_err(|e| format!("failed to write custom_steam_path: {e}"))?;
    println!("custom steam path saved: {}", path);
    Ok(())
}
