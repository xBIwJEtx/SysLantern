use zerocopy::{FromBytes, Immutable, KnownLayout};

pub const COMM_LEN: usize = 16;
pub const MAX_FILENAME_LEN: usize = 256;

#[repr(C)]
#[derive(FromBytes, Immutable, KnownLayout)]
pub struct RawExecEvent {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub comm: [u8; COMM_LEN],
    pub filename: [u8; MAX_FILENAME_LEN],
}