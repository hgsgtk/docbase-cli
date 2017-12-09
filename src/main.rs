#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
USAGE:
    docbase-cli
    docbase-cli config
    docbase-cli post <post-file-path>...
    docbase-cli (help | --version)

Options:
    -h, --help      Show this screen
    -v, --version   Show version
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_config: bool,
    cmd_post: bool,
    arg_post_file_repo: Vec<String>,
    flag_help: bool,
    flag_version: bool
}


fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
