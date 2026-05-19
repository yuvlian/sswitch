pub mod path;
pub mod skip_offline;
pub mod start;
pub mod stop;
pub mod switch;
pub mod tag;
pub mod tags;
pub mod untag;

pub fn print_help() {
    println!("sswitch || https://github.com/yuvlian/sswitch");
    println!();
    println!("usage:");
    println!("  sswitch                  list accounts, switch, and launch steam");
    println!(
        "  sswitch <tag>            instantly switch & launch steam to the account with this tag"
    );
    println!("  sswitch stop             stop the steam client");
    println!("  sswitch path <path>      set custom steam folder path");
    println!("  sswitch skip-offline     enable skipofflinemodewarning for all accounts");
    println!(
        "  sswitch tag <tag> <acc>  assign a tag to an account (<acc> can be index, username, display name, or steam id)"
    );
    println!("  sswitch untag <tag>      remove an existing tag");
    println!("  sswitch tags             list all configured tags");
}
