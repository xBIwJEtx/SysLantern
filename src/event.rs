use crate::raw::RawExecEvent;

fn cstr_to_string(bytes: &[u8]) -> String {
    let pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());

    String::from_utf8_lossy(&bytes[..pos]).into_owned()
}

#[derive(Debug, serde::Serialize)]
pub struct ExecEvent{
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub comm: String,
    pub filename: String,
}

impl From<&RawExecEvent> for ExecEvent {
    fn from(raw: &RawExecEvent) -> Self {
        Self{
            pid: raw.pid,
            ppid: raw.ppid,
            uid: raw.uid,

            comm: cstr_to_string(&raw.comm),
            filename: cstr_to_string(&raw.filename),
        }
    }
}