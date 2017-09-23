use std::io;

use chan_signal::Signal;
use libc;

pub fn kill(pid: i32, sig: Signal) -> io::Result<()> {
    let _todo = unsafe { libc::kill(pid, sig_int(sig)) };
    Ok(())
}

pub fn setsid() -> io::Result<()> {
    let _todo = unsafe { libc::setsid() };
    Ok(())
}

pub fn all_sig() -> Vec<Signal> {
    vec![
        Signal::HUP,
        Signal::INT,
        Signal::QUIT,
        Signal::ILL,
        Signal::ABRT,
        Signal::FPE,
        Signal::KILL,
        Signal::SEGV,
        Signal::PIPE,
        Signal::ALRM,
        Signal::TERM,
        Signal::USR1,
        Signal::USR2,
        Signal::CHLD,
        Signal::CONT,
        Signal::STOP,
        Signal::TSTP,
        Signal::TTIN,
        Signal::TTOU,
        Signal::BUS,
        Signal::PROF,
        Signal::SYS,
        Signal::TRAP,
        Signal::URG,
        Signal::VTALRM,
        Signal::XCPU,
        Signal::XFSZ,
        Signal::IO,
        Signal::WINCH,
    ]
}

fn sig_int(sig: Signal) -> libc::c_int {
    match sig {
        Signal::HUP => libc::SIGHUP,
        Signal::INT => libc::SIGINT,
        Signal::QUIT => libc::SIGQUIT,
        Signal::ILL => libc::SIGILL,
        Signal::ABRT => libc::SIGABRT,
        Signal::FPE => libc::SIGFPE,
        Signal::KILL => libc::SIGKILL,
        Signal::SEGV => libc::SIGSEGV,
        Signal::PIPE => libc::SIGPIPE,
        Signal::ALRM => libc::SIGALRM,
        Signal::TERM => libc::SIGTERM,
        Signal::USR1 => libc::SIGUSR1,
        Signal::USR2 => libc::SIGUSR2,
        Signal::CHLD => libc::SIGCHLD,
        Signal::CONT => libc::SIGCONT,
        Signal::STOP => libc::SIGSTOP,
        Signal::TSTP => libc::SIGTSTP,
        Signal::TTIN => libc::SIGTTIN,
        Signal::TTOU => libc::SIGTTOU,
        Signal::BUS => libc::SIGBUS,
        Signal::PROF => libc::SIGPROF,
        Signal::SYS => libc::SIGSYS,
        Signal::TRAP => libc::SIGTRAP,
        Signal::URG => libc::SIGURG,
        Signal::VTALRM => libc::SIGVTALRM,
        Signal::XCPU => libc::SIGXCPU,
        Signal::XFSZ => libc::SIGXFSZ,
        Signal::IO => libc::SIGIO,
        Signal::WINCH => libc::SIGWINCH,
        sig => panic!("unsupported signal: {:?}", sig),
    }
}
