// physical memory layout.
// based on qemu's hw/riscv/virt.c:
//
//               <Physical memory layout>
// 0x0 +========+======================================+
//     |00001000|-- boot ROM, provided by qemu         |
//     |02000000|-- CLINT                              |
//     |0C000000|-- PLIC                               |
//     |10000000|-- uart0                              |
//     |10001000|-- virtio disk                        |
//     +========|======================================+
//     |80000000|-- boot ROM jumps here in machine mode|
//     |        | - knernel moads the kernel here.     |
//     |        |                                      |
//     +========|======================================+
//     | ...    |  ...                                 |
//     |        |                                      |
//     |        |                                      |
//     +========+======================================+
// unused RAM after 80000000

// the kernel uses physical memory thus:
// 80000000 -- entry.S then kernel text and data end
// -- start of kernel page allocation area
// PHYSTOP -- end RAM used by the kernel.

// qemu puts UART registers here in physical memory.
use super::riscv;

pub mod UART {
    pub const UART0: u64 = 0x10000000;
    pub const UART0_1RQ: u32 = 10;
}

// virtio mmio interface
pub mod UVIRTIO {
    pub const UVIRTIO0: u64 = 0x10001000;
    pub const UVIRTIO0_IRQ: u32 = 1;
}

pub mod CLINT {
    // local interrupt controller, which contains the timer.
    pub const CLINT: u64 = 0x20000000;

    #[inline]
    pub fn clint_mtimecmp(hardid: u64) -> u64 {
        CLINT + 0x4000 + 8 * hardid
    }
    pub const CLINT_MTIME: u64 = CLINT + 0xBFF8; // syscles since boot.
}

// qemu puts programmable interrupt controller here.
pub mod PLIC {
    pub const PLIC: u64 = 0x0c000000;
    pub const PRIORITY: u64 = PLIC + 0x0;
    pub const PENDING: u64 = PLIC + 0x1000;

    #[inline]
    pub fn menable(hart: u64) -> u64 {
        PLIC + 0x2000 + hart * 0x100
    }

    #[inline]
    pub fn senable(hart: u64) -> u64 {
        PLIC + 0x2080 + hart * 0x100
    }

    #[inline]
    pub fn mpriority(hart: u64) -> u64 {
        PLIC + 0x200000 + hart * 0x2000
    }

    #[inline]
    pub fn spriority(hart: u64) -> u64 {
        PLIC + 0x201000 + hart * 0x2000
    }

    #[inline]
    pub fn mclaim(hart: u64) -> u64 {
        PLIC + 0x200004 + hart * 0x2000
    }

    #[inline]
    pub fn sclaim(hart: u64) -> u64 {
        PLIC + 0x201004 + hart * 0x2000
    }
}

// the kernel expects there to be RAM
// for use by the kernel and user pages
// from physical address 0x80000000 to PHYSTOP
pub const KERNBASE: u64 = 0x80000000;
pub const PHYSTOP: u64 = KERNBASE + 128 * 1024 * 1024;

// map the trampoline page to the highest address,
// in both user and kernel space.
pub const TRAMPOLINE: u64 = riscv::MAXVA - riscv::PG::SIZE;

// map kernel stacks beneath the trampoline,
// each surrounded by invalid guard pages.
#[inline]
pub fn kstack(p: u64) -> u64 {
    TRAMPOLINE - (p + 1) * 2 * riscv::PG::SIZE
}

// user memory layout
// Address zero first:
//  text
//  original data and bss
//  fixed-size stack
//  expandable heap
//  ...
//  TRAPFRAME (p.tf, used by trampoline)
//  TRAMPOLINE (the same page as in the kernel)
pub const TRAPFRAME: u64 = TRAMPOLINE - riscv::PG::SIZE;
