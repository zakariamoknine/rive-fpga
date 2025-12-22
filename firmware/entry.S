.section .text.boot

.global entry
entry:
	la      sp, stack_top
	call    start

.section .data

.p2align 4
.space 512
stack_top:
