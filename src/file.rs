use super::pipe;
use super::sleeplock::SleepLock;
use super::fs;
use super::params;

pub enum FileType {
    FdNode,
    FdPipe,
    FdInode,
    FdDevice,
}

#[derive(Default)]
pub struct OpenFileBufferes<'a>([File<'a>; params::NOFILE]);

#[derive(Default)]
pub struct File<'a> {
    pub tp: Option<FileType>,
    pub refc: i32, // reference count.
    pub readable: bool,
    pub writable: bool,
    pub pipe: Option<&'a pipe::Pipe<'a>>, // FdPipe
    pub ip: Option<&'a mut Inode<'a>>,        // FdInode and FdDevice
    pub off: u32,                 // FdInode
    pub major: i16,               // FdDevice
}

// in-memory copy of an inode
#[derive(Default)]
pub struct Inode<'a> {
    pub dev: u32,  // Device number
    pub inum: u32, // Inode numer
    pub refc: i32, // reference count
    pub sleep: SleepLock<'a>,   // protect everything below here
    pub valid: i32,             // inode has been read from disk?

    pub tp: u16,    // copy of disk inode
    pub major: u16,
    pub miner: u16,
    pub nlink: u16,
    pub size: u32,
    pub addrs: [u32; fs::NDIRECT],
}


