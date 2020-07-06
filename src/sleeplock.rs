// long  term lock for processes
use super::spinlock::SpinLock;

pub struct SleepLock<'a> {
    pub locked: bool,     // is the lock held?
    pub lk: SpinLock<'a>, // spinlock protecting this sleep lock.

    // debug
    pub name: &'a str,
    pub pid: u32,
}

impl<'a> SleepLock<'a> {
    pub fn new(name: &'a str) -> SleepLock<'a> {
        return SleepLock {
            name,
            lk: SpinLock::new("sleep lock"),
            locked: false,
            pid: 0,
        };
    }

    pub fn acquire(&mut self) {
        self.lk.acquire();
        while self.lk.locked {
            // TODO
        }
    }
}
