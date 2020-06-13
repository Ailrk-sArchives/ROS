use super::proc::{mycpu, Cpu};

pub struct SpinLock<'a> {
    pub locked: bool,
    pub name: &'a str,
    pub cpu: Option<&'a Cpu<'a>>,
}

impl<'a> SpinLock<'a> {
    fn new(name: &'a str) -> SpinLock<'a> {
        return SpinLock {
            name,
            locked: false,
            cpu: None,
        };
    }

    // acquire the lock.
    // Loops (spins) until the lock is acquired.
    fn acqure(&mut self) {
        push_off(); // disable interrupts to avoid deadlock.
        if self.holding() {
            panic!("acqure");
        }

        // on risc-v sync_lock_test_and_set turns into an atomic swap.
        while __sync_lock_test_and_set(self.locked, 1) != 0 {}

        __sync_synchronize();
        self.cpu = mycpu();
    }

    // check whether this cpu is holding the lock.
    fn holding(&self) -> bool {
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
fn push_off() {}

fn pop_off() {}
