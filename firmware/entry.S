.section .text.boot

.global entry
entry:
	/* Setup the global pointer */
	lla     gp, __global_pointer$

	/* Setup the stack */
	la      sp, firmware_stack

	/* Clear BSS */
	la      t0, __bss_start
	la      t1, __bss_end
1:
	sd      x0, 0(t0)
	addi	t0, t0, 8
	bltu	t0, t1, 1b

	/* Off we go */
	tail    start_firmware

hang:
	wfi
	j       1b

.section .data

.p2align 4
.space 512
firmware_stack:
