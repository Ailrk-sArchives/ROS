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
//      - its damn registers in the memory ...
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


// read hart id
#[inline]
pub fn r_mhartid() -> u64 {
    let x: u64;
    unsafe { asm!("csrr {}, mhartid", out(reg) x) }
    x
}

// mstatus. Machine Mode Status Register.
pub const MSTATUS_MPP_MASK: u64 = 3 << 11; // pp: previous mode.
pub const MSTATUS_MPP_M: u64 = 3 << 11; // set pp to machine-mode.
pub const MSTATUS_MPP_S: u64 = 1 << 11; // set pp to supervisor-mode.
pub const MSTATUS_MPP_U: u64 = 0 << 11; // set pp to user-mode.
pub const MSTATUS_MIE: u64 = 1 << 3; // machine mode iterrupt enable.

#[inline]
pub fn r_mstatus() -> u64 {
    let x: u64;
    unsafe { asm!("cssr {}, mstatus", in(reg) x) }
    x
}

#[inline]
pub fn w_mstatus(x: u64) {
    unsafe { asm!("csrw mstatus, {}", in(reg) x) }
}

/*
  mepc
  machine exception program counter
  holds the addr return to from an exception.
*/
#[inline]
pub fn w_mepc(x: u64) {
    unsafe { asm!("csrw mepc, {}", in(reg) x) }
}

// sstatus, Supervior Status Register.
pub const SSTATUS_SPP: u64 = 1 << 8; // previous mode. 1=supervisor 0=user
pub const SSTATUS_SPIE: u64 = 1 << 5; // supervisor previous interrupt enable
pub const SSTATUS_UPIE: u64 = 1 << 4; // user previous interrupt enable
pub const SSTATUS_SIE: u64 = 1 << 1; // supervisor interrupt enable
pub const SSTATUS_UIE: u64 = 1 << 1; // user interrupt enable


pub type Pte = u64;
pub type Pagetable<'a> = &'a u64;
