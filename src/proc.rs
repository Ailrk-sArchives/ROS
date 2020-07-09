// process and scheduling
// process -- unit of isolation.

use super::file::{Inode, OpenFileBufferes};
use super::params;
use super::riscv;
use super::spinlock;

#[derive(Default)]
pub struct Cpus<'a>([Cpu<'a>; params::NCPU]);

pub struct Procs<'a>([Proc<'a>; params::NPROC]);
impl<'a> Default for Procs<'a> {
    fn default() -> Self {
        Procs([Default::default(); params::NPROC])
    }
}
pub struct InitProc<'a>(Proc<'a>);

// registers for context swithing.
#[derive(Default)]
pub struct Context {
    pub ra: u64,
    pub sp: u64,

    // callee-saved
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
}

// state of each CPU
#[derive(Default)]
pub struct Cpu<'a> {
    pub proc: Option<&'a mut Proc<'a>>, // the process run on cpu.
    pub scheduler: Context,             // switch to enter scheduler.
    pub noff: i32,                      // depth of push_off() nesting.
    pub intena: bool,                   // interrups flag
}

// per-process data for trap for handling code in trampoline.S
// sits in a page by itself just under the trampoline page in the
// user page table. Not sepcially mapped in the kenrnel page table.
// The sscratch register points here.
// uservec in trampoline.S saves user registers in the trapframe,
// then initializes registers from the trapframe's kernel_sp,
// kernel_hartid, kernel_satp, and jumps to knernel_trap.
// usertrapert() and userret in tarmpoline.S set up the trapframe's
// kernal_*, restore user registers from the trapframe,
// switch to user page table, and enter user space,
// the trapframe incldues callee-saved user registers like s0-s11 because the
// return-to-user path via usertrapret() doesn't return through the
// entire knernel call stack.
#[derive(Default)]
pub struct Trapframe {
    pub kernel_satp: u64,   // kernal page table
    pub kernel_sp: u64,     // top of process's kernal stack
    pub kernel_trap: u64,   // unsertrap()
    pub epc: u64,           // saved user program counter.
    pub kernel_hartid: u64, // saved kernel tp

    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,

    pub t0: u64,
    pub t1: u64,
    pub t2: u64,

    pub s0: u64,
    pub s1: u64,

    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,

    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,

    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
}

#[derive(PartialEq)]
pub enum ProcState {
    Unused,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}
impl Default for ProcState {
    fn default() -> Self {
        ProcState::Unused
    }
}

// maintain important states for process.
// page table:      map from virtual address space to physical space
// knernel stack:   stack on kernel space for system call.
//                  - in riscv `ecall` make a syscall, which raise the hardware
//                    privilege level and change `pc` to knernel defined entry point.
//                    when syscall finished, kernel will switch back to user space by
//                    calling `sret` (lower hw privilege).
// run state.       for scheduling
#[derive(Default)]
pub struct Proc<'a> {
    os: Option<*mut State<'a>>,
    pub lock: spinlock::SpinLock<'a>,

    pub state: ProcState, // process state
    pub parent: Option<&'a Proc<'a>>,
    pub chan: Option<*const ()>, // if non zero, sleep on chan
    pub killed: bool,            // kill flag
    pub xstate: bool,            // exit status
    pub pid: i32,                // process id

    pub kstack: u64,                   // bottom of kernal stack for the process
    pub sz: usize,                     // size of proces mem
    pub pagetable: riscv::Pagetable,   // page table
    pub tf: Option<&'a mut Trapframe>, // data page for trampoline.S
    pub context: Context,              // switch() here to run process
    pub ofile: Option<&'a mut OpenFileBufferes<'a>>, // open files
    pub cwd: Option<&'a mut Inode<'a>>, // current directory.
    pub name: &'a str,                 // proc name (debugging)
}

// process can access the OS state.
impl<'a> OSFetch<'a> for Proc<'a> {
    fn get_os(&mut self) -> Option<*mut State<'a>> {
        self.os
    }
}

impl<'a> Proc<'a> {
    pub fn new() -> Proc<'a> {
        unimplemented!();
    }

    // Wake up process if it is sleeping in wait(); used by exit();
    fn wakeup1(&mut self) {
        if !self.lock.holding() {
            panic!("wakeup1");
        }
        if self.chan.unwrap() as *const () == self as *const _ as *const () {
            self.state = ProcState::Runnable;
        }
    }

    // Kill the process with the given pid
    // The victim won't exit until it tires to return
    // to user space (usertrap() in trap.rs)
}

pub fn sched() {}

pub enum StateErr {
    CpuIndexErr,
    ProcessDoesntExistErr,
}

// global state, exists for the entire lifetime of the program.
pub struct State<'a> {
    cpus: Cpus<'a>,
    procs: Procs<'a>,
    initproc: InitProc<'a>,
    nextpid: i32,
    pidLock: spinlock::SpinLock<'a>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        let mut state = State {
            cpus: Default::default(),
            procs: Default::default(),
            initproc: InitProc(Proc::new()),
            nextpid: 1,
            pidLock: Default::default(),
        };
        let stateptr = Some(&mut state as *mut State<'a>);
        state.pidLock = spinlock::SpinLock::new("nexPid", stateptr);
        state
    }

    // must be called with interrupts disabled.
    // prevent process be moved to a different cpu.
    pub fn cpuid(&self) -> u64 {
        riscv::REGS::TP::read()
    }

    pub fn mycpu(&mut self) -> Result<&'a mut Cpu<'_>, StateErr> {
        let id = self.cpuid() as usize;
        match self.cpus {
            // borrow Option content in Cpus
            Cpus(ref mut cpus) if id < cpus.len() => Ok(&mut cpus[id]),
            _ => Err(StateErr::CpuIndexErr),
        }
    }

    pub fn myproc(&mut self) -> Result<&'a mut Proc<'_>, StateErr> {
        unimplemented!();
    }

    pub fn allocpid(&mut self) -> i32 {
        self.pidLock.acquire();
        self.nextpid = self.nextpid + 1;
        let pid = self.nextpid;
        self.pidLock.release();
        pid
    }

    // atomically release lock and sleep on chan.
    // reacquires lock when awakened.
    pub fn sleep<T>(&mut self, chan: *const T, lk: &'a mut spinlock::SpinLock<'a>) {
        // must acquire p.lock in order to
        // change p.state and then call sched.
        // Once we hold p.lock we can be guaranteed that we won't
        // miss any wakeup
        // so it's okay to release lk.
        let p = self.proc_ref_mut();
        if lk != &p.lock {
            p.lock.acquire();
            lk.release();
        }

        p.chan = Some(chan as *const ());
        p.state = ProcState::Sleeping;

        sched();

        // no more chan
        p.chan = None;

        // reacquire original lock.
        if lk != &p.lock {
            p.lock.release();
            lk.acquire();
        }
    }

    // wake up all processes sleeping on chan.
    // Must be called without any p.lock.
    pub fn wakeup<T>(&mut self, chan: *const T) {
        self.procs.0.iter_mut().map(|proc| {
            proc.lock.acquire();
            if proc.state == ProcState::Sleeping
                && match proc.chan {
                    Some(chanp) => chanp == chan as *const (),
                    None => false,
                }
            {
                proc.state = ProcState::Runnable;
            }
            proc.lock.release();
        });
    }
}

impl<'a> OSFetch<'a> for State<'a> {
    fn get_os(&mut self) -> Option<*mut State<'a>> {
        return Some(self as *mut _);
    }
}

// fetch state element easier.
// all structs that can fetch os will have a definite size.
pub trait OSFetch<'a>: Sized {
    fn get_os(&mut self) -> Option<*mut State<'a>>;

    #[inline]
    fn os_ref(&mut self) -> &'a State<'a> {
        unsafe { self.get_os().unwrap().as_ref().unwrap() }
    }

    #[inline]
    fn os_ref_mut(&mut self) -> &'a mut State<'a> {
        unsafe { self.get_os().unwrap().as_mut().unwrap() }
    }

    #[inline]
    fn cpu_ref(&mut self) -> &'a Cpu<'a> {
        unsafe { (*self.get_os().unwrap()).mycpu().ok().unwrap() as &'_ Cpu<'_> }
    }

    #[inline]
    fn cpu_ref_mut(&mut self) -> &'a mut Cpu<'a> {
        unsafe { (*self.get_os().unwrap()).mycpu().ok().unwrap() }
    }

    #[inline]
    fn proc_ref(&mut self) -> &'a Proc<'a> {
        unsafe { (*self.get_os().unwrap()).myproc().ok().unwrap() as &'_ Proc<'_> }
    }

    #[inline]
    fn proc_ref_mut(&mut self) -> &'a mut Proc<'a> {
        unsafe { (*self.get_os().unwrap()).myproc().ok().unwrap() }
    }
}
