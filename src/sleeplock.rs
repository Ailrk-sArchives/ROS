// long  term lock for processes
use super::proc::{OSFetch, State};
use super::spinlock::SpinLock;

#[derive(Default)]
pub struct SleepLock<'a> {
    os: Option<*mut State<'a>>,
    pub locked: bool,     // is the lock held?
    pub lk: SpinLock<'a>, // spinlock protecting this sleep lock.

    // debug
    pub name: &'a str,
    pub pid: i32,
}

impl<'a> OSFetch<'a> for SleepLock<'a> {
    fn get_os(&mut self) -> Option<*mut State<'a>> {
        self.os
    }
}

impl<'a> SleepLock<'a> {
    pub fn new(name: &'a str, os: Option<*mut State<'a>>) -> SleepLock<'a> {
        SleepLock {
            name,
            lk: SpinLock::new("sleep lock", os),
            locked: false,
            pid: 0,
            os,
        }
    }

    pub fn acquire(&mut self) {
        self.lk.acquire();
        while self.lk.locked {
            self.proc_ref_mut().lock.acquire();
            self.lk.release();
        }
        self.lk.locked = true;
        self.pid = self.proc_ref().pid;
        self.lk.release();
    }

    pub fn release(&mut self) {
        self.lk.acquire();
        self.locked = false;
        self.pid = 0;
        self.os_ref_mut().wakeup(&self);
        self.lk.release();
    }

    pub fn holding(&mut self) -> bool {
        self.lk.acquire();
        let r: bool = self.locked && (self.pid == self.proc_ref_mut().pid);
        self.lk.release();
        r
    }
}
