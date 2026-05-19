use crate::common::{load_tags, save_tags};

pub fn run(tag_name: &str) -> Result<(), String> {
    let tag_name = tag_name.trim().to_lowercase();
    let (mut tags, cap) = load_tags();
    if tags.remove(&tag_name).is_some() {
        save_tags(&tags, cap)?;
        println!("removed tag '{}'", tag_name);
        Ok(())
    } else {
        Err(format!("tag '{}' not found", tag_name))
    }
}
