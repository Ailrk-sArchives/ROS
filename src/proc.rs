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
    pub intena: i32,                    // interrups flag
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

    pub parent: &'a Proc<'a>,
    pub procstate: ProcState, // process state
    pub killed: bool,         // kill flag
    pub xstate: bool,         // exit status
    pub pid: i32,             // process id

    kstack: u64,                               // bottom of kernal stack for the process
    sz: usize,                                 // size of proces mem
    pagetable: Pagetable,                  // page table
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
