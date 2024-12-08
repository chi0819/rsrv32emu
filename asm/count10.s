.global _start
.text

_start:
    la a0, counter
    lw t0, 0(a0)
    la a0, max
    lw t1, 0(a0)
_loop:
    bge t0, t1, _end
    addi t0, t0, 1
    j _loop
_end:
    la a0, counter
    sw t0, 0(a0)

.data
    counter: .word 0
    max:     .word 10
