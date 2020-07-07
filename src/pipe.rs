use super::spinlock;

const pipesize: usize = 512;
pub struct PipeData([u8; pipesize]);

impl Default for PipeData {
    fn default() -> Self {
        PipeData([0 as u8; 512])
    }
}

#[derive(Default)]
pub struct Pipe<'a> {
    spinlock: spinlock::SpinLock<'a>,
    data: PipeData,
    nread: u32,     // num of bytes read
    nwrite: u32,    // num of bytes written
    readopen: i32,  // read fd is still open
    writeopen: i32,  // write fd is still open
}

