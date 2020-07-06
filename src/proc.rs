// process and scheduling
// process -- unit of isolation.

use super::file::{File, Inode};
use super::params;
use super::riscv::Pagetable;
use super::spinlock;

// registers for context swithing.
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
pub struct Cpu<'a> {
    pub proc: Option<&'a mut Proc<'a>>, // the process run on cpu.
    pub scheduler: Context,             // switch to enter scheduler.
    pub noff: i32,                      // depth of push_off() nesting.
    pub intena: bool,                   // interrups flag
}

type Cpus<'a> = [Cpu<'a>; params::NCPU];

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

pub enum ProcState {
    Unused,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}

// maintain important states for process.
// page table:      map from virtual address space to physical space
// knernel stack:   stack on kernel space for system call.
//                  - in riscv `ecall` make a syscall, which raise the hardware
//                    privilege level and change `pc` to knernel defined entry point.
//                    when syscall finished, kernel will switch back to user space by
//                    calling `sret` (lower hw privilege).
// run state.       for scheduling
pub struct Proc<'a> {
    pub lock: spinlock::SpinLock<'a>,

    pub state: ProcState, // process state
    pub parent: &'a Proc<'a>,
    pub chan: Option<*const ()>, // if non zero, sleep on chan TODO chan should be ptr to any
    pub killed: bool,    // kill flag
    pub xstate: bool,    // exit status
    pub pid: i32,        // process id

    pub kstack: u64,           // bottom of kernal stack for the process
    pub sz: usize,             // size of proces mem
    pub pagetable: Pagetable,  // page table
    pub tf: &'a mut Trapframe, // data page for trampoline.S
    pub context: Context,      // switch() here to run process
    pub ofile: &'a mut [File<'a>; params::NOFILE], // open files
    pub cwd: &'a mut Inode,    // current directory.
    pub name: [u8; 16],        // proc name (debugging)
}

impl<'a> Proc<'a> {
    pub fn new() -> Proc<'a> {
        unimplemented!();
    }
}

pub fn cpuid() -> u32 {
    unimplemented!();
}

pub fn mycpu<'a>() -> &'a Cpu<'a> {
    unimplemented!();
}

pub fn myproc<'a>() -> &'a Proc<'a> {
    unimplemented!();
}

// atomically release lock and sleep on chan.
// reacquires lock when awakened.
pub fn sleep<'a>(chan: *const (), lk: &'a spinlock::SpinLock<'a>) {
    let mut p = myproc();

    // must acquire p.lock in order to
    // change p.state and then call sched.
    // Once we hold p.lock we can be guaranteed that we won't
    // miss any wakeup
    // so it's okay to release lk.
    if lk != &p.lock {
        p.lock.acquire();
        lk.release();
    }

    p.chan = Some(chan);
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
pub fn wakeup(chan: *const ()) {
}

pub fn sched() {

}


