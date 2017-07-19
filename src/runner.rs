use std::os::unix::process::CommandExt;
use std::process::{Command, Child, ExitStatus};
use std::thread;

use chan;
use chan::{Sender, Receiver};
use chan_signal::Signal;
use chan_signal;

use error::*;
use process;

#[derive(Clone)]
pub struct Runner {
    cmd: String,
    interval: Option<u32>,
}

impl Runner {
    pub fn new(cmd: String, interval: Option<u32>) -> Runner {
        Runner {
            cmd: cmd,
            interval: interval,
        }
    }

    pub fn start(&self) -> Result<Option<ExitStatus>> {
        let sig = chan_signal::notify(process::all_sig().as_slice());
        loop {
            let (sig_send, sig_recv) = chan::sync(0);
            let (stat_send, stat_recv) = chan::sync(0);
            let tick = chan::tick_ms(self.interval.unwrap_or(0));
            let self_clone = self.clone();
            thread::spawn(move || self_clone.run_proc(stat_send, sig_recv));
            let mut exit = false;
            chan_select! {
                tick.recv() => {
                    sig_send.send(Signal::INT);
                },
                sig.recv() -> sig => {
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

    fn run_proc(&self, stat_send: Sender<ExitStatus>, sig_recv: Receiver<Signal>) -> Result<()> {
        let mut child = self.spawn_proc()?;
        let pid = child.id() as i32;
        thread::spawn(move || sig_recv.recv().map(|sig| process::kill(-pid, sig)));
        stat_send.send(child.wait()?);
        Ok(())
    }

    fn spawn_proc(&self) -> Result<Child> {
        let mut command = Command::new("sh");
        command.arg("-c").arg(self.cmd.clone());
        Ok(command.before_exec(|| process::setsid()).spawn()?)
    }
}
