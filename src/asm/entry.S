.section .data
.global stack0
.section .text
.global start
.section .text
.global _entry

_entry:
        la sp, stack0
        li a0, 1024*4

        csrr a1, mhartid   # current hardid

        addi a1, a1, 1     # to index 1
        mul a0, a0, a1

# start() in start.rs
        call start
