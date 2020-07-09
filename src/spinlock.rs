use super::proc::{Cpu, OSFetch, State};
use super::riscv;

#[derive(Default)]
pub struct SpinLock<'a> {
    os: Option<*mut State<'a>>,
    pub locked: bool,
    pub name: &'a str,
    pub cpu: Option<&'a mut Cpu<'a>>,
}

// identity check. Compare pointer since rust doesn't have
// direct idenity
// you only care if two spinlock are the same lock.
impl<'a> PartialEq for SpinLock<'a> {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl<'a> OSFetch<'a> for SpinLock<'a> {
    fn get_os(&mut self) -> Option<*mut State<'a>> {
        self.os
    }
}

impl<'a> SpinLock<'a> {
    pub fn new(name: &'a str, os: Option<*mut State<'a>>) -> SpinLock<'a> {
        SpinLock {
            name,
            locked: false,
            cpu: None,
            os,
        }
    }

    // acquire the lock.
    // Loops (spins) until the lock is acquired.
    pub fn acquire(&mut self) {
        // disable interrupts to avoid deadlock.
        self.os_ref_mut().push_off();
        if self.holding() {
            panic!("acquire");
        }

        // on risc-v sync_lock_test_and_set turns into an atomic swap.
        // this is why it is called spinlock.
        while riscv::SYNC::lock_test_and_set(&mut self.locked, true) {}

        riscv::SYNC::synchronize();
        self.cpu = Some(self.cpu_ref_mut());
    }

    // release the lock
    pub fn release(&mut self) {
        if !self.holding() {
            panic!("release");
        }
        self.cpu = None;

        // this turns into a fence instruction in riscv.
        riscv::SYNC::synchronize();

        riscv::SYNC::lock_release(&mut self.locked);
        self.os_ref_mut().pop_off();
    }

    // check whether this cpu is holding the lock.
    pub fn holding(&mut self) -> bool {
        let mut r = false;
        self.os_ref_mut()
            .push_off()
            .sdo(|state| {
                let isSameCpu = self
                    .cpu
                    .as_mut()
                    .map(|val| {
                        let this = state.cpu_ref() as *const Cpu as *const ();
                        let other = val as &mut Cpu as *const Cpu as *const ();
                        this == other
                    })
                    .unwrap_or(false);
                r = self.locked && isSameCpu;
            })
            .pop_off();
        r
    }
}

// push_off / pop_off are like intr_off()/intr_on() except that they are
// matched:
// it takes two pop_off() to undo two push_off()s. Also, if interrrupts
// are initially off, then push_off, pop_off leaves them off.
impl<'a> State<'a> {
    pub fn push_off(&mut self) -> &mut Self {
        let old = riscv::DEV_INTR::get();
        riscv::DEV_INTR::off();
        if self.cpu_ref().noff == 0 {
            self.cpu_ref_mut().intena = old;
        }
        self.cpu_ref_mut().noff += 1;
        self
    }

    pub fn pop_off(&mut self) -> &mut Self {
        let c = self.cpu_ref_mut();
        if riscv::DEV_INTR::get() {
            panic!("pop off - interruptible");
        }
        c.noff -= 1;
        if c.noff < 0 {
            panic!("pop off");
        }
        if c.noff == 0 && c.intena {
            riscv::DEV_INTR::on();
        }
        self
    }

    // perform side effects here.
    pub fn sdo<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> (),
    {
        f(self);
        self
    }
}
