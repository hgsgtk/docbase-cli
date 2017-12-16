#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
to use DocBase cli
USAGE:
    docbase-cli
    docbase-cli config
    docbase-cli post <post-file-path>...
    docbase-cli (-h | --help)
    docbase-cli --version

Options:
    -h, --help      Show this screen.
    --version       Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_config: bool,
    cmd_post: bool,
    arg_post_file_repo: Vec<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.options_first(true).deserialize())
//    .and_then(|a: Args| {
//        let _args = Args {
//        }
//        Ok( _args )
//    })
    .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
