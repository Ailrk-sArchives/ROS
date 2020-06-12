# execute on machine mode.
#  - on the booting process intitially paging hardware is closed.
#    so virtual address will map to physical address directly.
#
# - entry will setup a stack for rust code to run.

.section .data
.global stack0
.section .text
.global start
.section .text
.global _entry

_entry:
        la sp, stack0      # stack0 is the initial stack for bootstrap.
        li a0, 1024*4

        csrr a1, mhartid   # current hardid

        addi a1, a1, 1     # to index 1
        mul a0, a0, a1

# start() in start.rs
        call start

# some RISCV asm refereces

#####################################################################
# - labels                                                          #
#####################################################################
#   loop:                                                           #
#       j loop                                                      #
#                                                                   #
#####################################################################
# - abs addressing                                                  #
#####################################################################
#   .section text                                                   #
#   .global _start                                                  #
#   _start:                                                         #
#              lui  a0,       %hi(msg)  # load msg(hi)              #
#              addi a0,  a0,  %lo(msg)  # load msg(lo)              #
#              jal ra, puts                                         #
#   2:         j 2b                                                 #
#   .section .rodata                                                #
#   msg:                                                            #
#              .string "Hellow world\n"                             #
#####################################################################
#  - relative addressing                                            #
#####################################################################
#   .section .text                                                  #
#   .globl _start                                                   #
#   _start:                                                         #
#   1:	    auipc a0,     %pcrel_hi(msg) # load msg(hi)             #
#   	    addi  a0, a0, %pcrel_lo(1b)  # load msg(lo)             #
#   	    jal ra, puts                                            #
#   2:	    j 2b                                                    #
#                                                                   #
#   .section .rodata                                                #
#   msg:                                                            #
#   	    .string "Hello World\n"                                 #
#                                                                   #
#####################################################################
#  - load immediate                                                 #
#####################################################################
#   .section .text                                                  #
#   .globl _start                                                   #
#   _start:                                                         #
#                                                                   #
#   .equ CONSTANT, 0xdeadbeef                                       #
#                                                                   #
#           li a0, CONSTANT                                         #
#####################################################################
#  - load address                                                   #
#####################################################################
#   .section .text                                                  #
#   .globl _start                                                   #
#   _start:                                                         #
#                                                                   #
#           la a0, msg                                              #
#                                                                   #
#   .section .rodata                                                #
#   msg:                                                            #
#       .string "Hello World\n"                                     #
#####################################################################
#  - csr. an example on show to enable timer interrups              #
#####################################################################
#    .equ RTC_BASE,      0x40000000                                 #
#    .equ TIMER_BASE,    0x40004000                                 #
#                                                                   #
#    # setup machine trap vector                                    #
#    1:      auipc   t0, %pcrel_hi(mtvec)        # load mtvec(hi)   #
#            addi    t0, t0, %pcrel_lo(1b)       # load mtvec(lo)   #
#            csrrw   zero, mtvec, t0                                #
#                                                                   #
#    # set msatus.MIE=1 (enable M mode interrupt)                   #
#            li      t0, 8                                          #
#            csrrs   zero, mstatus, t0                              #
#                                                                   #
#    # set mie.MTIE=1 (enable M mode timer interrupts)              #
#            li      t0, 128                                        #
#            csrrs   zero, mie, t0                                  #
#                                                                   #
#    # read from mtime                                              #
#            li      a0, RTC_BASE                                   #
#            ld      a1, 0(a0)                                      #
#                                                                   #
#    # write to mtimecmp                                            #
#            li      a0, TIMER_BASE                                 #
#            li      t0, 1000000000                                 #
#            add     a1, a1, t0                                     #
#            sd      a1, 0(a0)                                      #
#                                                                   #
#    # loop                                                         #
#    loop:                                                          #
#            wfi                                                    #
#            j loop                                                 #
#                                                                   #
#    # break on interrupt                                           #
#    mtvec:                                                         #
#            csrrc  t0, mcause, zero                                #
#            bgez t0, fail       # interrupt causes are less than zero
#            slli t0, t0, 1      # shift off high bit               #
#            srli t0, t0, 1                                         #
#            li t1, 7            # check this is an m_timer interrupt
#            bne t0, t1, fail                                       #
#            j pass                                                 #
#                                                                   #
#    pass:                                                          #
#            la a0, pass_msg                                        #
#            jal puts                                               #
#            j shutdown                                             #
#                                                                   #
#    fail:                                                          #
#            la a0, fail_msg                                        #
#            jal puts                                               #
#            j shutdown                                             #
#                                                                   #
#    .section .rodata                                               #
#                                                                   #
#    pass_msg:                                                      #
#            .string "PASS\n"                                       #
#                                                                   #
#    fail_msg:                                                      #
#            .string "FAIL\n"t                                      #
#####################################################################
