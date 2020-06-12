#![feature(asm)]

mod riscv;
mod proc;
mod params;
mod spinlock;
mod file;
mod pipe;
mod memlayout;
mod string;
mod vm;

fn main() {
    println!("Hello, world!");
}
