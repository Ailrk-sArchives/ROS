use super::proc::{mycpu, Cpu};
use super::riscv;

pub struct SpinLock<'a> {
    pub locked: bool,
    pub name: &'a str,
    pub cpu: Option<&'a Cpu<'a>>,
}

pub impl<'a> SpinLock<'a> {
    pub fn new(name: &'a str) -> SpinLock<'a> {
        return SpinLock {
            name,
            locked: false,
            cpu: None,
        };
    }

    // acquire the lock.
    // Loops (spins) until the lock is acquired.
    pub fn acqure(&mut self) {
        push_off(); // disable interrupts to avoid deadlock.
        if self.holding() {
            panic!("acqure");
        }

        // on risc-v sync_lock_test_and_set turns into an atomic swap.
        while riscv::SYNC::lock_test_and_set(&mut self.locked, true) {}

        riscv::SYNC::synchronize();
        self.cpu = Some(mycpu());
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
        pop_off();
    }

    // check whether this cpu is holding the lock.
    pub fn holding(&self) -> bool {
        push_off();
        let r = self.locked
            && self
                .cpu
                .map(|val| (val as *const Cpu) == (mycpu() as *const Cpu))
                .unwrap_or(false);
        pop_off();
        r
    }
}

// push_off / pop_off are like intr_off()/intr_on() except that they are
// matched:
// it takes two pop_off() to undo two push_off()s. Also, if interrrupts
// are initially off, then push_off, pop_off leaves them off.

fn push_off() {
    let old = riscv::DEV_INTR::get();
    riscv::DEV_INTR::off();
    if (mycpu().noff == 0) {
        mycpu().intena = old;
    }
    mycpu().noff += 1;
}

fn pop_off() {
    let c = mycpu();
    if (riscv::DEV_INTR::get()) {
        panic!("pop off - interruptible");
    }
    c.noff -= 1;

    if (c.noff < 0) {
        panic!("pop off");
    }

    if (c.noff == 0 && c.intena) {
        riscv::DEV_INTR::on();
    }
}
