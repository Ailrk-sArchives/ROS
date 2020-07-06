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

fn main() {
    println!("Hello, world!");
}
