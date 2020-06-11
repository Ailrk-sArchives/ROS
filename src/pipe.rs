use super::spinlock;

const pipesize: usize = 512;

pub struct Pipe<'a> {
    spinlock: spinlock::SpinLock<'a>,
    data: [u8; pipesize],
    nread: u32,     // num of bytes read
    nwrite: u32,    // num of bytes written
    readopen: u32,  // read fd is still open
    writeopen: u32,  // write fd is still open
}

