use crate::os::SteamPlatform;

pub fn run(platform: &dyn SteamPlatform) -> Result<(), String> {
    platform.kill_steam()?;
    println!("steam processes terminated.");
    Ok(())
}
