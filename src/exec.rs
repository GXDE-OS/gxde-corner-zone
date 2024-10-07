use crate::edge::Edge;
use crate::config::Commands;
use std::ffi::CString;
use std::ffi::CStr;
use nix::unistd;
use nix::sys;

pub fn run(edge: Edge, cmds: &Commands, block: bool, debug: bool) {
    let cmd = match edge {
        Edge::TOPLEFT => &cmds.topleft,
        Edge::TOPRIGHT => &cmds.topright,
        Edge::BOTTOMRIGHT => &cmds.bottomright,
        Edge::BOTTOMLEFT => &cmds.bottomleft,
        Edge::LEFT => &cmds.left,
        Edge::TOP => &cmds.top,
        Edge::RIGHT => &cmds.right,
        Edge::BOTTOM => &cmds.bottom,
        _ => &None,
    };

    if debug {
        println!("{:?}: {:?}", edge, cmd);
    }

    if cmd.is_none() {
        return;
    }

    let s: String = cmd.as_ref().unwrap().to_string();
    let split: Vec<&str> = s.split_whitespace().collect();
    if split.is_empty() {
        return;
    }
    let vec: Vec<CString> = split
        .iter()
        .map(|s| CString::new(s.as_bytes()).unwrap())
        .collect();
    let args: Vec<&CStr> = vec.iter().map(|c| c.as_c_str()).collect();

    unsafe {
        match unistd::fork() {
            Ok(unistd::ForkResult::Child) => {
                let _ = unistd::execvp(args[0], &args);
                libc::_exit(0);
            }
            Ok(unistd::ForkResult::Parent { .. }) => {
                if block {
                    let _ = sys::wait::wait();
                }
            }
            Err(_) => println!("Fork failed"),
        }
    }
}
