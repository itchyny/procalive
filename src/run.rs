use std::os::unix::process::CommandExt;
use std::process::{Command, Child, ExitStatus};
use std::thread;

use chan;
use chan::{Sender, Receiver};
use chan_signal::Signal;
use chan_signal;

use error::*;
use process;

pub fn run(cmd: String) -> Result<()> {
    let sig = chan_signal::notify(process::all_sig().as_slice());
    loop {
        let (sig_send, sig_recv) = chan::sync(0);
        let cmd = cmd.clone();
        let (done_send, done_recv) = chan::sync(0);
        thread::spawn(move || {
            println!("Exit status: {:?}", run_proc(done_send, sig_recv, cmd));
        });
        chan_select! {
            sig.recv() -> sig => {
                println!("Received: {:?}", sig);
                sig_send.send(sig.unwrap());
                return Ok(())
            },
            done_recv.recv() => {
                println!("Restarting...");
            }
        }
    }
}

fn run_proc(_: Sender<()>, sig_recv: Receiver<Signal>, cmd: String) -> Result<ExitStatus> {
    let mut child = spawn_proc(cmd)?;
    let pid = child.id() as i32;
    thread::spawn(move || sig_recv.recv().map(|sig| process::kill(-pid, sig)));
    let status = child.wait()?;
    Ok(status)
}

fn spawn_proc(cmd: String) -> Result<Child> {
    let mut command = Command::new("sh");
    command.arg("-c").arg(cmd);
    Ok(command.before_exec(|| process::setsid()).spawn()?)
}
