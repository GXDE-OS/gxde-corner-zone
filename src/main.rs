mod config;
mod signal;
mod edge;
mod exec;
mod utils;

use signal::RUNNING;
use edge::{Edge, in_edge};
use exec::run;
use utils::{query_pointer, get_screen_size};
use structopt::StructOpt;
use std::time::{Duration, Instant};

#[derive(Debug, StructOpt)]
#[structopt(name = "gxde-corner-zone", about = "GXDE corner zone hot spot manager")]
struct Opts {
    #[structopt(short, long)]
    block: bool,

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, default_value = "400")]
    timeout: u64,

    #[structopt(short, long, default_value = "0")]
    offset: i32,
}

fn main() {
    let opts = Opts::from_args();
    signal::setup_signals();

    let cmds = config::load_commands();
    let (xmax, ymax) = get_screen_size();
    let timeout = Duration::from_millis(opts.timeout);

    let mut prev_edge = Edge::NONE;
    let mut triggered_edge = Edge::NONE;
    let mut start_time = Instant::now();

    while RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        let (x, y) = query_pointer();

        let curr_edge = in_edge(x, y, xmax, ymax, opts.offset);

        if curr_edge != Edge::NONE && curr_edge == prev_edge && curr_edge != triggered_edge {
            if start_time.elapsed() > timeout {
                run(curr_edge, &cmds, opts.block, opts.debug);
                triggered_edge = curr_edge;
            }
        } else if curr_edge != prev_edge {
            prev_edge = curr_edge;
            triggered_edge = Edge::NONE;
            start_time = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
