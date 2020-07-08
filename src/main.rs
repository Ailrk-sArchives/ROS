#![feature(asm)]

mod riscv;
mod proc;
mod params;
mod spinlock;
mod file;
mod fs;
mod pipe;
mod memlayout;
mod string;
mod vm;
mod buf;
mod sleeplock;
mod bio;
mod switch;

// singleton state of the entire os.
static mut STATE: proc::State = proc::State::new();

fn main() {
    let mut os = proc::State::new();

}
