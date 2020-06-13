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

#####################################################################
# some RISCV asm refereces
# - registers: 32 register
#   - x0: zero reg
#   - ra: return address
#   - sp: stack pointer
#   - gp: global pointer
#   - tp: thread pointer
#   - t0 - t2: temp reg
#   - fp: frame pointer
#   - s1: saved reg
#   - a0 - a7: function arguments
#   - s2 - s11: saved reg
#   - t3 - t6: temp reg
#   - pc program counter

#######################################################################
# Pseudo Ops
##########################################################################
#  name         arguments              descrption
##########################################################################
# .align          <int>                 align to power of 2
# .file        "filename"               emit filename FILE LOCAL symbol table
# .globl       symbol_name              emit symbol_name to symbol table (scope GLOBAL)
# .local       symbol_name              emit symbol_name to symbol table (scope LOCAL)
# .comm    sym_name, size, align        emit common object to .bss section
# .ident        "string"                accepted for source compatibility
# .section {.text,.data,.rodata,.bss}   emit section (if not present default .text)
# .size         sym, sym                accepted for source compatibility
# .text
# .rodata
# .data
# .bss
# .string        "string"               emit string
# .asciz         "string"               emit string (alias for .string)
# .equ         name, value              constant definition
# .macro      name arg1 [,argn]         begin macro
# .endm                                 end macro
# .type       symbol, @function         accepted for source cokpatibililty
# .option{rvc,norvc,pic,nopic,push,pop} riscv options
# .byte    expression [, expression]*   8 bit words
# .2byte          ...                   16 bit words
# .half           ...                   16 bit words
# .short          ...                   16 bit words
# .4byte          ...                   32 bit words
# .word           ...                   32 bit words
# .long           ...                   32 bit words
# .8byte          ...                   64 bit words
# .dword          ...                   64 bit words
# .quad           ...                   64 bit words
# .zero           ...                   zero bytes
#####################################################################

# assember relocation functions/expansions
##########################################################################
#  asm notion           descrption           instruction            comment
##########################################################################
#  %hi(sym)             absolute(HI20)          lui              load upper immediate
#  %lo(sym)             absolute(LO12)      load, store, add
# %pcerl_hi(sym)     pc relative(HI20)         auipc            add upper immediate to pc
# %pcerl_lo(sym)     pc relative(LO12)      load, store, add
#####################################################################

# list instructions
######################################################################
# lui    rd, imm                  // load upper immediate.
#                                 //load the upper 20 bits pad the rest 12 with 0
# x[rd] = sext(immediate[31:12] << 12)
######################################################################
# auipc  rd, imm                  // add upper imm to pc.
#                                 // build pc relavie addr
# x[rd] = pc + sext(immediate[31:12] << 12)
######################################################################
# addi  rd, rs1, imm              // add immediate
# x[rd] = x[rs1] + sext(immediate)
######################################################################
# stli  rd, rs1, imm               // set less than imm
#  x[rx] = x[rs1] < s sext(immediate)
######################################################################
# stliu  rd, rs1, imm
# xori  rd, rs1, imm
# ori  rd, rs1, imm
# andi rd, rs, imm
######################################################################
# slli  rd, rs1, shamt              // left shift imm
# x[rd] = x[rs1] << shamt
#
# srli  rd, rs, shamt
######################################################################
# add  rd, rs1, rs2
# x[rd] = x[rs1] = x[rs2]
# sub  rd, rs1, rs2
# sll rd, rs1 rs2                    // left shift
# sllu rd, rs1 rs2                   // left shift
# srl rd, rs1 rs2                    // right shift
# srlu rd, rs1 rs2                   // right shift
# or rd, rs1, rs2
# and rd, rs1, rs2
######################################################################
# fence pred, succ                  // used to order device io and memory
#                                   // access as viewd by other RISCV harts
#                                   // ad external devices or coprocessors.
#                                   // like sync primitive
# Fence(pred, succ)
#
# fence.i                           // explicit synchoronization between writes
#                                   // to instructin memory and instruction fetches
#                                   // on the same hart.
######################################################################
# // CSR instructions atomically read-modify-write a single CSR.
# // all this operations are kinda bidirectional, and not evey intuitive.
# csrrw rd, offset, rs1             // atomic read/write csr
#                                   // 1. csr value will be read, then write into rd
#                                   // 2. rs1 value will be read, then write into csr
#                                   // 3. it do a value swap
# // this two beheaves the same
# csrrs rd, offset, rs1             // atomic read and set bits in csr
#                                   // 1. csr value will be rad, then write to rd
#                                   // 2. rs1 is treated as a bit mask that specifies
#                                   //    what position to be set.
# csrrc  rd, offset, rs1            // atomic read and clear bits in csr
#                                   // 1. csr value will be rad, then write to rd
#                                   // 2. rs1 is treated as a bit mask that specifies
#                                   //    what position to be cleared.
# csrrwi rd, offset, uimm
# csrrsi rd, offset, uimm
# csrrci rd, offset, uimm
######################################################################
# ecall                             // make a request to the supporting exe env. (EE)
# RaiseException(EnvCall)
######################################################################
# ebreak                            // used by debugger to cause control to be transffered
#                                   // to a debugging environment.
# RaiseException(BreakPoint)
######################################################################
# uret                              // return from traps in U-mode, and uret copies upie
#                                   // into uie, then sets upie
# ExceptionReturn(User)
# sret
# mret
######################################################################
# wfi                               // wait for interrupt
#                                   // provides a hint to the implementation that
#                                   // current hart can be stalled until an interrupt
#                                   // might need servicing.
# while (noInterruptsPending) idle
######################################################################
# sfence.vma                        // guarantees that any previous stores
#                                   // already visible to current RISCV hart are
#                                   // ordered before all subsequent implicit
#                                   // references from that hart to the memory-management
#                                   // data structure.
######################################################################
# lb  rd, offset(rs)
# lh  rd, offset(rs)
# lw  rd, offset(rs)
# lbu  rd, offset(rs)
# lhu  rd, offset(rs)
# sb  rd, offset(rs)
# sh  rd, offset(rs)
# sw  rd, offset(rs)
# jal  rd, offset
# jalr rd, rs1, offset
# beq  rs1, rs2, offset
# bne  rs1, rs2, offset
# blt  rs1, rs2, offset
# bgt  rs1, rs2, offset
#####################################################################

#####################################################################
# floating point rounding modes
#####################################################################
# fcvt.w.s a0, fa0, --
# where -- can be
#   rne: round to nearest, ties to even
#   rtz: round towards zero
#   rdn: round down
#   rup: round up
#   rmm: round to nearest, ties to max magnitude
#   dyn: dynamic roudning mode (the mode specified in frm field in fscr regiser is used)
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
#   .section .rodata                                                #
#   msg:                                                            #
#   	    .string "Hello World\n"                                 #
#####################################################################
#  - load immediate                                                 #
#####################################################################
#   .section .text                                                  #
#   .globl _start                                                   #
#   _start:                                                         #
#   .equ CONSTANT, 0xdeadbeef                                       #
#           li a0, CONSTANT                                         #
#####################################################################
#  - load address                                                   #
#####################################################################
#   .section .text                                                  #
#   .globl _start                                                   #
#   _start:                                                         #
#           la a0, msg                                              #
#   .section .rodata                                                #
#   msg:                                                            #
#       .string "Hello World\n"                                     #
#####################################################################
#  - csr. an example on show to enable timer interrups              #
#####################################################################
#    .equ RTC_BASE,      0x40000000                                 #
#    .equ TIMER_BASE,    0x40004000                                 #
#    # setup machine trap vector                                    #
#    1:      auipc   t0, %pcrel_hi(mtvec)        # load mtvec(hi)   #
#            addi    t0, t0, %pcrel_lo(1b)       # load mtvec(lo)   #
#            csrrw   zero, mtvec, t0                                #
#    # set msatus.MIE=1 (enable M mode interrupt)                   #
#            li      t0, 8                                          #
#            csrrs   zero, mstatus, t0                              #
#    # set mie.MTIE=1 (enable M mode timer interrupts)              #
#            li      t0, 128                                        #
#            csrrs   zero, mie, t0                                  #
#    # read from mtime                                              #
#            li      a0, RTC_BASE                                   #
#            ld      a1, 0(a0)                                      #
#    # write to mtimecmp                                            #
#            li      a0, TIMER_BASE                                 #
#            li      t0, 1000000000                                 #
#            add     a1, a1, t0                                     #
#            sd      a1, 0(a0)                                      #
#    # loop                                                         #
#    loop:                                                          #
#            wfi                                                    #
#            j loop                                                 #
#    # break on interrupt                                           #
#    mtvec:                                                         #
#            csrrc  t0, mcause, zero                                #
#            bgez t0, fail       # interrupt causes are less than zero
#            slli t0, t0, 1      # shift off high bit               #
#            srli t0, t0, 1                                         #
#            li t1, 7            # check this is an m_timer interrupt
#            bne t0, t1, fail                                       #
#            j pass                                                 #
#    pass:                                                          #
#            la a0, pass_msg                                        #
#            jal puts                                               #
#            j shutdown                                             #
#    fail:                                                          #
#            la a0, fail_msg                                        #
#            jal puts                                               #
#            j shutdown                                             #
#    .section .rodata                                               #
#    pass_msg:                                                      #
#            .string "PASS\n"                                       #
#    fail_msg:                                                      #
#            .string "FAIL\n"t                                      #
#####################################################################
