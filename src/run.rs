use std::process::Command;
use itertools::Itertools;

use error::*;

pub fn run<I>(mut args: I) -> Result<()>
    where I: Iterator<Item = String>
{
    let cmd = args.join(" ");
    if cmd.len() < 1 {
        return Err(Error::NoCommand);
    }
    let mut child = Command::new("sh").arg("-c").arg(cmd).spawn()?;
    let ecode = child.wait()?;
    Ok(())
}
