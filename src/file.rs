use super::pipe;

pub enum FileType {
    FdNode,
    FdPipe,
    FdInode,
    FdDevice,
}

pub struct File<'a> {
    pub tp: FileType,
    pub refc: i32, // reference count.
    pub readable: bool,
    pub writable: bool,
    pub pipe: &'a pipe::Pipe<'a>, // FdPipe
    pub ip: &'a mut Inode,        // FdInode and FdDevice
    pub off: u32,                 // FdInode
    pub major: i16,               // FdDevice
}

// in-memory copy of an inode
pub struct Inode {}
