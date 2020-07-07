// long  term lock for processes
use super::spinlock::SpinLock;
use super::proc::{sleep, wakeup, State};

pub struct SleepLock<'a> {
    pub locked: bool,     // is the lock held?
    pub lk: SpinLock<'a>, // spinlock protecting this sleep lock.

    // debug
    pub name: &'a str,
    pub pid: i32,
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
            sleep(&self, &mut self.lk);
        }
        self.lk.locked = true;
        self.pid = myproc().pid;
        self.lk.release();
    }

    pub fn release(&mut self) {
        self.lk.acquire();
        self.locked = false;
        self.pid = 0;
        wakeup(&self);
        self.lk.release();
    }

    pub fn holding(&mut self) -> bool {
        self.lk.acquire();
        let r: bool = self.locked && (self.pid == myproc().pid);
        self.lk.release();
        r
    }
}
