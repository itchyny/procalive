use std::process::{Command, Child, ExitStatus};
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
    loop {
        run_proc(cmd.clone())?;
    }
}

fn run_proc(cmd: String) -> Result<ExitStatus> {
    let mut child = spawn_proc(cmd)?;
    let status = child.wait()?;
    Ok(status)
}

fn spawn_proc(cmd: String) -> Result<Child> {
    let mut command = Command::new("sh");
    command.arg("-c").arg(cmd);
    Ok(command.before_exec(|| setsid()).spawn()?)
}

fn setsid() -> io::Result<()> {
    let _todo = unsafe { libc::setsid() };
    Ok(())
}
