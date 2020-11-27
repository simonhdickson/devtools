use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Simon Dickson <simonhdickson@users.noreply.github.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Base64(Base64),
}

#[derive(Clap)]
struct Base64 {
    #[clap(short)]
    decode: bool,
    
    input: String,
}


fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Base64(params) => {
            let mut m = devtools_core::Base64::default();

            if params.decode {
                m.set_base64(&params.input).unwrap();
                println!("{}", m.get_plain_text());
            } else {
                m.set_plain_text(&params.input);
                println!("{}", m.get_base64());
            }
        }
    }
}
