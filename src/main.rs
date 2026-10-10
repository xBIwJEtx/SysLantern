mod collector;
mod skel {
    include!(concat!(env!("OUT_DIR"), "/syslantern.skel.rs"));
}
mod event;
mod raw;
mod output;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))?;

    collector::run(running)
}