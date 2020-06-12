// manage page table and address space.
// page table (implemented by hardware) provides the ability to map a virtual address
// to physical address.
// Process can only see their virtual address space. This is how processes achieve memory
// isolation.
//
// - layout of virtual address space of a user process
//         MAXVA -> +===============+
//                  |  trampoline   |  for switching to knernel
//                  +===============+
//                  |  trapframe    |
//                  +===============+
//                  |               |
//                  |    heap       |
//                  |               |
//                  |               |
//                  +===============+
//                  |   user stack  |
//                  +===============+
//                  | user text and |
//                  | data          |  instruction come first, then global variales
//             0 -> +===============+
// use 39 - 1 bits for virtual address, maxium address = 2^38 - 1 = 0x3fffffff = MAXVA

use super::riscv;


pub fn kvminit() {

}
