mod cmds;
mod common;
mod os;

use common::{load_tags, print_simple_accounts_list};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let platform = os::sys::CurrentPlatform;

    let result = if args.len() > 1 {
        match args[1].as_str() {
            "stop" => cmds::stop::run(&platform),
            "path" => {
                if args.len() < 3 {
                    eprintln!("usage: sswitch path <path>");
                    std::process::exit(0);
                }
                cmds::path::run(&args[2])
            }
            "skip-offline" => cmds::skip_offline::run(&platform),
            "start" => cmds::start::run(&platform),
            "tag" => {
                if args.len() < 4 {
                    eprintln!("usage: sswitch tag <tag> <acc>\n");
                    eprintln!("where <acc> can be:");
                    eprintln!("  - the number of the account in the list below (e.g. 1)");
                    eprintln!("  - the login username (e.g. gabelogannewell)");
                    eprintln!("  - the display name (e.g. Gabe Newell)");
                    eprintln!("  - the steam ID (e.g. 76561197960287930)");
                    print_simple_accounts_list(&platform);
                    std::process::exit(0);
                }
                cmds::tag::run(&platform, &args[2], &args[3])
            }
            "untag" => {
                if args.len() < 3 {
                    eprintln!("usage: sswitch untag <tag>");
                    print_simple_accounts_list(&platform);
                    std::process::exit(0);
                }
                cmds::untag::run(&args[2])
            }
            "tags" => cmds::tags::run(&platform),
            other => {
                let (tags, _) = load_tags();
                if tags.contains_key(&other.to_lowercase()) {
                    cmds::switch::handle_tag_switch(&platform, other)
                } else {
                    cmds::print_help();
                    Ok(())
                }
            }
        }
    } else {
        cmds::start::run(&platform)
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
