use super::sleeplock::SleepLock;
use super::fs::BSIZE;


pub struct Buf<'a> {
    valid: bool, // has data been read from disk?
    disk: bool,  // does disk own buffer?
    dev: u32,
    blockno: u32,
    lock: SleepLock<'a>,
    refcnt: u32,
    prev: &'a Buf<'a>, // LRU cache list
    next: &'a Buf<'a>,
    qnext: &'a Buf<'a>, // disk queue
    data: [char; BSIZE],
}
