// Buffer cache
//
// Buffer cache is a linked list of Buf struct holding
// cached copies of disk block content. Caching disk blocks
// in memory reduces the number of disk reads and also
// provides a synchronization point for disk blocks sused by
// muliple processed.
//
// Interface:
// To get a buffer from a particular disk block, call bread.
// After changing buffer data, call bwrite to write ito disk.
// When done with the buffer, call brelse.
// Do not use buffer after calling brelse.
// Only one process at a time can use a buffer, so do not
// keep them longer than necessary.


use super::spinlock::SpinLock;
use super::buf::Buf;
use super::params::NBUF;


pub struct Bcache<'a> {
    lock: SpinLock<'a>,
    buf: [Buf<'a>; NBUF],

    // linked list of all buffers, through prev/next.
    //head.next is mru.
    head: Buf<'a>,
}


impl<'a> Bcache<'a> {
    fn new() {
    }
}
