mod cmds;
mod common;
mod os;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let platform = os::sys::CurrentPlatform;

    match args.get(1).map(String::as_str) {
        None | Some("start") => cmds::start::run(&platform),

        Some("stop") => cmds::stop::run(&platform),

        Some("path") => {
            let path = args
                .get(2)
                .ok_or_else(|| "usage: sswitch path <path>".to_string())?;

            cmds::path::run(path)
        }

        Some("skip-offline") => cmds::skip_offline::run(&platform),

        Some("tag") => {
            let tag = args.get(2).ok_or_else(|| {
                format!(
                    "usage: sswitch tag <tag> <acc>\n\n{}",
                    common::account_help_text(&platform)
                )
            })?;

            let acc = args.get(3).ok_or_else(|| {
                format!(
                    "usage: sswitch tag <tag> <acc>\n\n{}",
                    common::account_help_text(&platform)
                )
            })?;

            cmds::tag::run(&platform, tag, acc)
        }

        Some("untag") => {
            let tag = args.get(2).ok_or_else(|| {
                format!(
                    "usage: sswitch untag <tag>\n\n{}",
                    common::account_help_text(&platform)
                )
            })?;

            cmds::untag::run(tag)
        }

        Some("tags") => cmds::tags::run(&platform),

        Some(other) => {
            let (tags, _) = common::load_tags();

            if tags.contains_key(&other.to_lowercase()) {
                cmds::switch::handle_tag_switch(&platform, other)
            } else {
                cmds::print_help();
                Ok(())
            }
        }
    }
}
