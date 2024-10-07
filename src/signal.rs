use std::sync::atomic::{AtomicBool, Ordering};

pub static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn sighandler() {
    RUNNING.store(false, Ordering::Relaxed);
}

pub fn setup_signals() {
    unsafe {
        libc::signal(libc::SIGINT, sighandler as usize);
        libc::signal(libc::SIGTERM, sighandler as usize);
        libc::signal(libc::SIGHUP, sighandler as usize);
    }
}
