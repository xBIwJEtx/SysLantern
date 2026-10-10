use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{ErrorKind, RingBufferBuilder};
use zerocopy::FromBytes;

use crate::event::ExecEvent;
use crate::output;
use crate::raw::RawExecEvent;
use crate::skel::SyslanternSkelBuilder;

pub fn run(running: Arc<AtomicBool>) -> Result<()> {
    let builder = SyslanternSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();

    let open_skel = builder
        .open(&mut open_object)
        .context("failed to open BPF object")?;
    let mut skel = open_skel.load().context("failed to load BPF program")?;
    skel.attach().context("failed to attach BPF program")?;

    let mut rb_builder = RingBufferBuilder::new();
    rb_builder
        .add(&skel.maps.events, handle_event)
        .context("failed to register ring buffer callback")?;
    let ringbuf = rb_builder.build().context("failed to build ring buffer")?;

    eprintln!("attached, press Ctrl+C to stop");

    while running.load(Ordering::SeqCst) {
        if let Err(e) = ringbuf.poll(Duration::from_millis(100)) {
            if e.kind() == ErrorKind::Interrupted {
                continue;
            }
            return Err(e).context("ring buffer poll failed");
        }
    }

    Ok(())
}

fn handle_event(data: &[u8]) -> i32 {
    let raw = match RawExecEvent::ref_from_bytes(data) {
        Ok(raw) => raw,
        Err(_) => {
            eprintln!(
                "skipping malformed event: got {} bytes, expected {}",
                data.len(),
                std::mem::size_of::<RawExecEvent>()
            );
            return 0;
        }
    };

    let event = ExecEvent::from(raw);

    match output::json::print_event(&event) {
        Ok(()) => 0,
        Err(e) => -e.raw_os_error().unwrap_or(5),
    }
}