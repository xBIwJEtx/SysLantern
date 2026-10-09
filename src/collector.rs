use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};

use crate::skel::SyslanternSkelBuilder;

pub fn run(running: Arc<AtomicBool>) -> Result<()> {
    let builder = SyslanternSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();

    let open_skel = builder
        .open(&mut open_object)
        .context("failed to open BPF object")?;
    let mut skel = open_skel.load().context("failed to load BPF program")?;
    skel.attach().context("failed to attach BPF program")?;

    eprintln!("attached, press Ctrl+C to stop");
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}