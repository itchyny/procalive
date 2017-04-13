#[macro_use]
extern crate clap;
extern crate itertools;
use itertools::Itertools;
extern crate shell_escape;

extern crate procalive;
use procalive::run::*;

fn main() {
    let matches = clap_app!(procalive =>
        (version: "0.0.1")
        (author: "itchyny <itchyny@hatena.ne.jp>")
        (about: "Keep your process alive!")
        (@arg COMMAND: +required +multiple "Command to execute."))
        .get_matches();
    let args: Vec<&str> = matches.values_of("COMMAND").unwrap().collect();
    let cmd = if args.len() == 1 {
        args[0].into()
    } else {
        args.into_iter().map(|s| shell_escape::escape(s.into())).join(" ")
    };
    match run(cmd) {
        Ok(status) => {
            status.and_then(|s| s.code()).map(|code| std::process::exit(code));
        }
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(2);
        }
    }
}
