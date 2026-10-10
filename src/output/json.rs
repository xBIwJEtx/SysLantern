use std::io::{self, Write};

use crate::event::ExecEvent;

pub fn print_event(event: &ExecEvent) -> io::Result<()> {
    let line = serde_json::to_string(event).expect("serialize event");
    let mut out = io::stdout().lock();
    writeln!(out, "{}", line)
}