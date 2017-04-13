#[macro_use]
extern crate clap;
extern crate itertools;
use itertools::Itertools;
extern crate shell_escape;

extern crate procalive;
use procalive::runner::*;

fn main() {
    let matches = clap_app!(procalive =>
        (version: "0.0.1")
        (author: "itchyny <itchyny@hatena.ne.jp>")
        (about: "Keep your process alive!")
        (@arg INTERVAL: --interval +takes_value "The interval (in seconds) to restart the process.")
        (@arg COMMAND: +required +multiple "Command to execute."))
        .get_matches();
    let interval_msec: Option<u32> = matches.value_of("INTERVAL")
        .and_then(|x| x.parse().ok())
        .map(|sec: f64| (sec * 1000.0) as u32);
    let args: Vec<&str> = matches.values_of("COMMAND").unwrap().collect();
    let cmd = if args.len() == 1 {
        args[0].into()
    } else {
        args.into_iter().map(|s| shell_escape::escape(s.into())).join(" ")
    };
    match Runner::new(cmd, interval_msec).start() {
        Ok(status) => {
            status.and_then(|s| s.code()).map(|code| std::process::exit(code));
        }
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(2);
        }
    }
}
