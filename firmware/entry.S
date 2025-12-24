	.section .text.boot

	.global entry
entry:
	/* Setup the global pointer */
	lla     gp, __global_pointer$

	/* Setup the stack */
	lla     sp, firmware_stack

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

	/* Keep the stack very small */
	.section .data
	.p2align 4
	.space 512
firmware_stack:

	/* Embed the fdt in the firmware image */
	.section .rodata, "a"
	.balign 8
	.global __dtb
__dtb:
	.incbin "rive-fpga.dtb"
	.global __dtb_end
__dtb_end:
