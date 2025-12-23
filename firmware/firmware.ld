OUTPUT_ARCH(riscv)
ENTRY(entry)

SECTIONS
{
	. = 0x00004000;

	.text : {
		KEEP(*(.text.boot))
		*(.text .text.*)
	}

	.rodata : {
		. = ALIGN(16);
		*(.srodata .srodata.*)
		. = ALIGN(16);
		*(.rodata .rodata.*)
	}

	.data : {
		. = ALIGN(16);
		__global_pointer$ = . + 0x800;
		*(.sdata .sdata.*)
		. = ALIGN(16);
		*(.data .data.*)
	}

	PROVIDE(__bss_start = .);
	.bss : {
		. = ALIGN(16);
		*(.sbss .sbss.*)
		. = ALIGN(16);
		*(.bss .bss.*)
	}
	PROVIDE(__bss_end = .);
}
