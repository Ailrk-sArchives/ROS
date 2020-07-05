// process and scheduling
// process -- unit of isolation.

use super::file::{File, Inode};
use super::params;
use super::riscv::Pagetable;
use super::spinlock;

// registers for context swithing.
#[repr(C)]
pub struct Context {
    ra: u64,
    sp: u64,

    // callee-saved
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
}

// state of each CPU
pub struct Cpu<'a> {
    proc: Option<&'a mut Proc<'a>>, // the process run on cpu.
    scheduler: Context,             // switch to enter scheduler.
    noff: i32,                      // depth of push_off() nesting.
    intena: i32,                    // interrups flag
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
    kernel_satp: u64,   // kernal page table
    kernel_sp: u64,     // top of process's kernal stack
    kernel_trap: u64,   // unsertrap()
    epc: u64,           // saved user program counter.
    kernel_hartid: u64, // saved kernel tp

    ra: u64,
    sp: u64,
    gp: u64,
    tp: u64,

    t0: u64,
    t1: u64,
    t2: u64,

    s0: u64,
    s1: u64,

    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,

    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,

    t3: u64,
    t4: u64,
    t5: u64,
    t6: u64,
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

    pub parent: &'a Proc<'a>,
    pub procstate: ProcState, // process state
    pub killed: bool,         // kill flag
    pub xstate: bool,         // exit status
    pub pid: i32,             // process id

    kstack: u64,                               // bottom of kernal stack for the process
    sz: usize,                                 // size of proces mem
    pagetable: Pagetable<'a>,                  // page table
    tf: &'a mut Trapframe,                     // data page for trampoline.S
    context: Context,                          // switch() here to run process
    ofile: &'a mut [File<'a>; params::NOFILE], // open files
    cwd: &'a mut Inode,                        // current directory.
    name: [u8; 16],                            // proc name (debugging)
}

impl<'a> Proc<'a> {
    fn new() -> Proc<'a> {
        unimplemented!();
    }


}

pub fn cpuid() -> u32 {
    unimplemented!();
}

pub fn mycpu<'a>() -> &'a Cpu<'a> {
    unimplemented!();
}
