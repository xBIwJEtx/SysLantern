use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(FromBytes, Immutable, KnownLayout)]
pub struct RawExecEvent {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub comm: [u8; 16],
    pub filename: [u8; 256],
}