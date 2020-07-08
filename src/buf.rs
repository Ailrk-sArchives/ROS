use super::fs::BSIZE;
use super::sleeplock::SleepLock;

#[derive(Default)]
pub struct Buf<'a> {
    valid: bool, // has data been read from disk?
    disk: bool,  // does disk own buffer?
    dev: u32,
    blockno: u32,
    lock: SleepLock<'a>,
    refcnt: u32,
    prev: Option<&'a Buf<'a>>, // LRU cache list
    next: Option<&'a Buf<'a>>,
    qnext: Option<&'a Buf<'a>>, // disk queue
    data: [char; BSIZE],
}
