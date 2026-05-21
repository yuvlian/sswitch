pub mod path;
pub mod skip_offline;
pub mod start;
pub mod stop;
pub mod switch;
pub mod tag;
pub mod tags;
pub mod untag;

pub fn print_help() {
    print!(
"sswitch || https://github.com/yuvlian/sswitch

usage:
  sswitch                  list accounts, switch, and launch steam
  sswitch <tag>            instantly switch & launch steam to the account with this tag
  sswitch stop             stop the steam client
  sswitch path <path>      set custom steam folder path
  sswitch skip-offline     enable skipofflinemodewarning for all accounts
  sswitch tag <tag> <acc>  assign a tag to an account (<acc> can be index, username, display name, or steam id)
  sswitch untag <tag>      remove an existing tag
  sswitch tags             list all configured tags
"
    );
}
