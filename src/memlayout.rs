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
pub const uart0: u64 =  0x10000000;
pub const uart0_1rq: u32 = 10;

// virtio mmio interface
pub const uvirtio0: u64 =  0x10001000;
pub const uvirtio0_irq: u32 =  1;

// local interrupt controller, which contains the timer.
pub const clint: u64 = 0x20000000;

#[inline]
pub fn clint_mtimecmp(hardid: u64) -> u64 {
    clint + 0x4000 + 8 * hardid
}

pub const clint_mtime: u64 = clint + 0xBFF8;  // syscles since boot.

// qemu puts programmable interrupt controller here.
