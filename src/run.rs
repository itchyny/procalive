use std::os::unix::process::CommandExt;
use std::process::{Command, Child, ExitStatus};
use std::thread;

use chan;
use chan::{Sender, Receiver};
use chan_signal::Signal;
use chan_signal;

use error::*;
use process;

pub fn run(cmd: String) -> Result<Option<ExitStatus>> {
    let sig = chan_signal::notify(process::all_sig().as_slice());
    loop {
        let (sig_send, sig_recv) = chan::sync(0);
        let cmd = cmd.clone();
        let (stat_send, stat_recv) = chan::sync(0);
        thread::spawn(move || run_proc(stat_send, sig_recv, cmd));
        let mut exit = false;
        chan_select! {
            sig.recv() -> sig => {
                println!("Received: {:?}", sig);
                sig_send.send(sig.unwrap());
                exit = true;
            },
            stat_recv.recv() => {
                println!("Restarting...");
            }
        }
        if exit {
            return Ok(stat_recv.recv());
        }
    }
}

fn run_proc(stat_send: Sender<ExitStatus>, sig_recv: Receiver<Signal>, cmd: String) -> Result<()> {
    let mut child = spawn_proc(cmd)?;
    let pid = child.id() as i32;
    thread::spawn(move || sig_recv.recv().map(|sig| process::kill(-pid, sig)));
    stat_send.send(child.wait()?);
    Ok(())
}

fn spawn_proc(cmd: String) -> Result<Child> {
    let mut command = Command::new("sh");
    command.arg("-c").arg(cmd);
    Ok(command.before_exec(|| process::setsid()).spawn()?)
}
