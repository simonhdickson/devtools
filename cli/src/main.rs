use std::io::Read;

use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Simon Dickson <simonhdickson@users.noreply.github.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Base64 {
        #[clap(short)]
        decode: bool,

        input: Option<String>,
    },
}

fn read_stdin() -> String {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    input
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Base64 { decode, input } => {
            let mut m = devtools_core::Base64::default();

            let input = input.unwrap_or_else(||read_stdin());

            if decode {
                m.set_base64(&input).unwrap();
                println!("{}", m.get_plain_text());
            } else {
                m.set_plain_text(&input);
                println!("{}", m.get_base64());
            }
        }
    }
}
