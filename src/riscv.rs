// inline wrapper of riscv.
// Some infor about RISCV
// - some terms:
//      core: processing unit support multiple riscv compatible harts.
//      harts: hardware threads.
//      accelerator: non programmable fixed function unit.

// riscv privileged software stack, level from high to low:
//      Application
//      ABI: application binary interface. abstraction of AEE.
//      AEE: aplication execution env. |  OS: operating system
//      SBI: supervisor binary interface.
//      SEE: supervisor execution env. (e.g. boot loader or BIOS)
//      Hypervisor: with hypearvior its possible to run multiple os.
//      HBI:  ...
//      HEE   ...

// more references are at the bottom ...

// read hart id
pub mod MHARTID {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, mhartid", out(reg) x);
        }
        x
    }
}

// mstatus. Machine Mode Status Register.
pub mod MSTATUS {
    pub const MPP_MASK: u64 = 3 << 11; // pp: previous mode.
    pub const MPP_M: u64 = 3 << 11; // set pp to machine-mode.
    pub const MPP_S: u64 = 1 << 11; // set pp to supervisor-mode.
    pub const MPP_U: u64 = 0 << 11; // set pp to user-mode.
    pub const MIE: u64 = 1 << 3; // machine mode iterrupt enable.
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("cssr {}, mstatus", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mstatus, {}", in(reg) x);
        }
    }
}

// mepc
// machine exception program counter
// holds the addr return to from an exception.
pub mod MEPC {
    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mepc, {}", in(reg) x);
        }
    }
}

// sstatus, Supervior Status Register.
pub mod SSTATUS {
    pub const SPP: u64 = 1 << 8; // previous mode. 1=supervisor 0=user
    pub const SPIE: u64 = 1 << 5; // supervisor previous interrupt enable
    pub const UPIE: u64 = 1 << 4; // user previous interrupt enable
    pub const SIE: u64 = 1 << 1; // supervisor interrupt enable
    pub const UIE: u64 = 1 << 1; // user interrupt enable
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64 = 0;
        unsafe {
            asm!("csrr {}, sstatus", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw sstatus, {}", in(reg) x);
        }
    }
}

// supervisor Interrupt pending
pub mod SIP {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, sip", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw sip, {}", in(reg) x);
        }
    }
}

// supervisor interrupt enable
pub mod SIE {
    pub const SEIE: u64 = 1 << 9; // external
    pub const STIE: u64 = 1 << 5; // timer
    pub const SSIE: u64 = 1 << 1; // software

    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, sie", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw sie, {}", in(reg) x);
        }
    }
}

// macine mode interrupt enable
pub mod MIE {
    pub const MEIE: u64 = 1 << 9; // external
    pub const MTIE: u64 = 1 << 5; // timer
    pub const MSIE: u64 = 1 << 1; // software
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, mie", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mie, {}", in(reg) x);
        }
    }
}

// machine exception program counter.
// holds the instruction address to which a return from
// exception will go
pub mod SEPC {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, sepc", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw sepc, {}", in(reg) x);
        }
    }
}

// machine exception delegation
pub mod MEDELEG {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, medeleg", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw medeleg, {}", in(reg) x);
        }
    }
}

// machine interrupt delegation
pub mod MIDELEG {
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, mideleg", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mideleg, {}", in(reg) x);
        }
    }
}

// supervisor trap-vector base address (vector location)
// low two bits ar mode.
pub mod STVEC {
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, stvec", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw stvec, {}", in(reg) x);
        }
    }
}

// machine mode interrupt vector
pub mod MTVEC {
    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mtvec, {}", in(reg) x);
        }
    }
}

pub mod SAPT {
    // use riscv's sv39 page table scheme.
    const SAPT_SV39: u64 = 8 << 60;

    #[inline]
    pub fn make(pagetable: u64) {
        SAPT_SV39 | (pagetable >> 12)
    }

    // supervisor address translation and protection.
    // holds the address of the page table.
    pub fn read() -> u64 {
        let mut x: u64;
        unsafe {
            asm!("csrr {}, satp", out(reg) x);
        }
        x
    }

    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw satp, {}", in(reg) x);
        }
    }
}

// supervisor scratch register, for early trap handler in trampoline.S
pub mod SSCRATCH {
    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw sscratch, {}", in(reg) x);
        }
    }

    #[inline]
    pub fn read(x: u64) {
        unsafe {
            asm!("csrw mscratch, {}", in(reg) x);
        }
    }
}

// supervisor Trap cause
pub mod SCAUSE {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64 = 0;
        unsafe {
            asm!("csrr {}, scause", out(reg) x);
        }
        x
    }
}

// supervisor trap value
pub mod STVAL {
    #[inline]
    pub fn read() -> u64 {
        let mut x: u64 = 0;
        unsafe {
            asm!("csrr {}, stval", out(reg) x);
        }
        x
    }
}

// machine mode counter enable
pub mod MCOUNTEREN {
    #[inline]
    pub fn write(x: u64) {
        unsafe {
            asm!("csrw mcounteren, {}", in(reg) x);
        }
    }

    #[inline]
    pub fn read() -> u64 {
        let mut x: u64 = 0;
        unsafe {
            asm!("csrr {}, mcounteren", out(reg) x);
        }
        x
    }
}

// machine mode cycle counter
pub mod TIME {
    pub fn read() -> u64 {
        let mut x: u64 = 0;
        unsafe {
            asm!("csrr {}, time", out(reg) x);
        }
        x
    }
}

pub mod DEV_INTR {
    // enable device interrupts
    use super::SIE;
    use super::SSTATUS;

    pub fn on() {
        SIE::write(SIE::read() | SIE::SEIE | SIE::STIE | SIE::SSIE);
        SSTATUS::write(SSTATUS::read() | SSTATUS::SIE);
    }

    // disable device interrupts
    pub fn off() {
        SSTATUS::write(SSTATUS::read() & !SSTATUS::SIE);
    }

    // check if device interrupts is enabled
    pub fn get() -> bool {
        let x = SSTATUS::read();
        (x & SSTATUS::SIE) != 0
    }
}

pub mod REGS {
    pub mod SP {
        pub fn read() -> u64 {
            let mut x: u64 = 0;
            unsafe {
                asm!("mv {}, sp", out(reg) x);
            }
            x
        }
    }

    pub mod TP {
        // read and write tp, the thread pointer
        // tp holds this core's hartid, the index into cpus[].
        pub fn read() -> u64 {
            let mut x: u64 = 0;
            unsafe {
                asm!("mv {}, tp", out(reg) x);
            }
            x
        }

        pub fn write(x: u64) {
            unsafe {
                asm!("mv tp, {}", in(reg) x);
            }
        }
    }

    pub mod RA {
        pub fn read() -> u64 {
            let mut x: u64 = 0;
            unsafe {
                asm!("mv {}, ra", out(reg) x);
            }
            x
        }
    }
}

pub mod FENCE {
    // flush the TLB.
    pub fn sfence_vma() {
        unsafe {
            asm!("sfence.vma zero, zero");
        }
    }
}

pub mod PG {
    pub const SIZE: u64 = 4096;
    pub const SHIFT: u64 = 12;

    #[inline]
    pub fn roundup(sz: u64) -> u64 {
        (sz + SIZE - 1) & !(SIZE - 1)
    }

    #[inline]
    pub fn rounddown(a: u64) -> u64 {
        a & !(SIZE - 1)
    }
}

pub mod PTE {
    pub const V: u64 = 1 << 0; // valid
    pub const R: u64 = 1 << 1; // valid
    pub const W: u64 = 1 << 2; // valid
    pub const X: u64 = 1 << 3; // valid
    pub const U: u64 = 1 << 4; // valid

    // shift a physical address to the right place for a PTE.
    #[inline]
    pub fn pa2pte(pa: u64) -> u64 {
        (pa >> 12) << 10
    }

    #[inline]
    pub fn pte2pa(pte: u64) -> u64 {
        (pte >> 10) << 12
    }

    #[inline]
    pub fn pte_flags(pte: u64) -> u64 {
        pte & 0x3FF
    }
}

// extact the three 9-bit page table indices from a virtual address.
pub mod PX {
    pub const PXMASK: u64 = 0x1ff; // 9 bit

    #[inline]
    pub fn pxshift(level: u64) -> u64 {
        super::PG::SHIFT + (9 * level)
    }

    #[inline]
    pub fn px(level: u64, va: u64) -> u64 {
        (va >> pxshift(level)) & PXMASK
    }
}

// one beyond the highest possible virtual address.
// MAXVA is actually one bit less than the max allowed by
// Sv39, to avoid having to sign-extend virtual addresses
// that have the high bit set.
pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 - 1);

// Atomic Memory Operations, perform read-modify-write operation
// for multiprocessor synchoronization.
// Instruction in form:
//      amo* rd r2 (r1)
// procedure:
//  rd <-data from address r1
//  rd <- rd op r2
//  r1 <- rd
//  where op is an binary operation based on amo instruction used.

pub mod SYNC {
    // atomic swap wrapper mimic gcc extension.
    // write value into p, and returns the previous value of p;
    // .aq bit set to ensure no other thread will observe the
    // AMO operation after AMO memory access.
    pub fn lock_test_and_set(p: &mut bool, value: bool) -> bool {
        let prevp: bool = *p;
        let ptemp: mut u64 = prevp as u64;
        unsafe {
            asm!(
                "
                li t0, {value}
                lw s1, {0}
                amoswap.w.aq t1, t0, (s1)
                 ",
                 inout("s1") ptemp,  // address
                 value = const value as u64
            );
        }
        *p = ptemp as bool;
        prevp
    }

    // sync all operations.
    // fence rwio, rwio is a conservative fence, it
    // will sync both memory access and device io operations.
    // more to read: https://github.com/riscv/riscv-gcc/pull/55
    pub fn synchronize() {
        unsafe {
            asm!("fence rwio, rwio");
        }
    }

    // Release the lock.
    // same as spinlock.locked = 0;
    // .rl bit set to ensure threads will not
    // observe the AMO operation before the AMO memory access.
    pub fn lock_release(p: &mut bool) {
        unsafe {
            asm!("la s1 {0}
                  amoswap.w.rl zero, zero, (s1)",
                  in(reg) *p as u64);
        }
    }
}

pub type Pte = u64;
pub type Pagetable = [u64; 512]; // 512 PTEs

// About qemu
//      - the os runs on quemu -machine virt

// monolithic kernel vs microkernel
//      - monolithic: entire os resides in kernel. thus all system calls
//        are in supervisor mode.
//        - interface bewtten different parts of the os are comlex.
//        - a error in monolithic kernal always happens in kernal mode.
//          so it is more risky to shut kernel down.
//     - microkernel: some functionalities are move to user space
//         - kernel code is much smaller.

// privilege levels
//      - hart running as a mode of one or more CSR (control and status reg)
//      - RISC-V privilege levels
//         +=======+==========+==============+=====+
//         | level | encoding |   name       | abbr|
//         |=======|==========|==============|=====|
//         |  0    |  00      |   user       |  U  |
//         |  1    |  01      | supervisor   |  S  |
//         |  2    |  10      | hypearvior   |  H  |
//         |  3    |  11      |  machine     |  M  |
//         +=======+==========+==============+=+===+
//      - previlege levels provide protection between different components.
//        of software stack.
//      - unpremiteed operation will raise an exception, which might
//        cause trap.
//      - `M` is the only mandatory mode for riscv platform. all privilege
//        granted.
//
// - Control and status register - CSR (mem mapped reg, 12 bit address)
//      - its registers in the memory ...
//      - some important regs.
//      - * = u/s/m, implies the privilege mode.
//
//         +============================================+
//     ->  ||   u/s/m   Trap setup                     ||
//         +============+===============================+
//         |   *status  | status regiser                |
//         |------------|-------------------------------|
//         |    *ie     | interrupt enable register     |
//         |------------|-------------------------------|
//         |   *tvec    | trap interrupt handler base   |
//         |            | address                       |
//         +============================================+
//     ->  ||       m   Trap setup                     ||
//         +============+===============================+
//         |   misa     | ISA and extensions            |
//         |------------|-------------------------------|
//         |   medeleg  | machine exception delegation r|
//         |------------|-------------------------------|
//         |   mideleg  | machine interrupt d~ r        |
//         +============================================+
//     ->  ||  u/s/m    Trap handling                  ||
//         +============================================+
//         |   *epc     | exception program counter     |
//         |------------|-------------------------------|
//         |  *scratch  | scratch reg for trap handlers |
//         |------------|-------------------------------|
//         |  *cause    | trap cause                    |
//         |------------|-------------------------------|
//         |  *badaddr  | bad address                   |
//         |------------|-------------------------------|
//         |    *ip     | interrupt pending             | (interrupt notifi)
//         +============================================+
//     ->  ||      m    machine info reg               ||
//         +============================================+
//         |  mvendorid | vendor id                     |
//         |------------|-------------------------------|
//         |  marchid   | arachitecture id              |
//         |------------|-------------------------------|
//         |  mimpid    | implementation id             |
//         |------------|-------------------------------|
//         |  mhartid   | hardware thread id            |
//         +============================================+
//     ->  ||      m  machine protection & translation ||
//         +============================================+
//         |  mbase     | base register                 |
//         |------------|-------------------------------|
//         |  mbound    | bound register                |
//         |------------|-------------------------------|
//         |  mibase    | instr base register           |
//         |------------|-------------------------------|
//         |  mibound   | instr bound register          |
//         |------------|-------------------------------|
//         |  mdbase    | data base regiter             |
//         |------------|-------------------------------|
//         |  mdbound   | data bound register           |
//         +============================================+
//     ->  ||    s supervisoer protection & translation||
//         +============================================+
//         |  sptbr     | page table base register      |
//         |============|===============================|

//
// - Platform level interrupt controller (PLIC)
//      - PLIC connects `global interrupt source` (usually io device)
//        to `interrupt target` (usually hart context).
//      - PLIC contains multiple interrupt gateways, one per interrupt
//        source.
//      - PLIC core performs interrupt prioritization and routing.
//      - flow:
//          (source)
//          >- global interrupts -> (interrupt gateway)
//          >- interrupt request -> {
//              (PLIC core)
//              >- latches -> (IP / interrupt pending bits in core)
//              >- select -> (IE / interrupt enable bits in core)
//          }
//          >- forward interrupt notification -> (targets)
//
//          (target)
//          >- interrupt completion -> (associate gateway)
//      - in a nutshell, `PLIC core` set reg and forward message to
//        target. `interrupt getway` dispatch messags.
//      - local interrupt source
//          - each hart has a number of local interrupt sources that
//            do not pass to PLIC.
//          - for instance, standard software interrupt, timer interrupt.
//          - local interrupts are serviced quickly since low latency
//            between source and servicing hart. (mcause)
//       - global interrupt source are those that are prioritize
//
//      - interrupt target and hart context
//          - interrupt targets are usually hart context.
//          - hart context: give privilege mode on a give hart.
