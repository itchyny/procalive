use std::process::{Command, Child};
use std::os::unix::process::CommandExt;
use itertools::Itertools;
use libc;

use error::*;

pub fn run<I>(mut args: I) -> Result<()>
    where I: Iterator<Item = String>
{
    let cmd = args.join(" ");
    if cmd.len() < 1 {
        return Err(Error::NoCommand);
    }
    let mut child = spawn_proc(cmd)?;
    let ecode = child.wait()?;
    Ok(())
}

fn spawn_proc(cmd: String) -> Result<Child> {
    let mut command = Command::new("sh");
    command.arg("-c").arg(cmd);
    Ok(command.before_exec(|| {
            let _ = unsafe { libc::setsid() };
            Ok(())
        })
        .spawn()?)
}
