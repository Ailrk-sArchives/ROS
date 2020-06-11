// read hart id
#[inline]
pub fn r_mhartid() -> u64 {
    let x: u64;
    unsafe { asm!("csrr {}, mhartid", out(reg) x) }
    x
}

// mstatus. Machine Mode Status Register.
pub const MSTATUS_MPP_MASK: u64 = 3 << 11;  // pp: previous mode.
pub const MSTATUS_MPP_M: u64 = 3 << 11;     // set pp to machine-mode.
pub const MSTATUS_MPP_S: u64 = 1 << 11;     // set pp to supervisor-mode.
pub const MSTATUS_MPP_U: u64 = 0 << 11;     // set pp to user-mode.
pub const MSTATUS_MIE: u64 = 1 << 3;        // machine mode iterrupt enable.

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
pub const SSTATUS_SPP: u64 = 1 << 8;  // previous mode. 1=supervisor 0=user
pub const SSTATUS_SPIE: u64 = 1 << 5; // supervisor previous interrupt enable
pub const SSTATUS_UPIE: u64 = 1 << 4; // user previous interrupt enable
pub const SSTATUS_SIE: u64 = 1 << 1;  // supervisor interrupt enable
pub const SSTATUS_UIE: u64 = 1 << 1;  // user interrupt enable


#[inline]
pub fn r_sstatus() -> u64 {

}


pub type Pte = u64;
pub type Pagetable<'a> = &'a u64;
