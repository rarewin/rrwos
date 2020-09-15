MEMORY
{
	rom (rx) : ORIGIN = 0x00000000, LENGTH = 16M
	ram (rwx): ORIGIN = 0x40000000, LENGTH = 128M
}

SECTIONS
{
	.text 0x40010000 : ALIGN(0x40)
	{
		*(.text);
		*(.text.*);
	} > ram

	.rodata : ALIGN(0x40)
	{
		*(.rodata);
		*(.rodata.*);
	} > ram

	.data : ALIGN(0x40)
	{
		*(.data);
	} > ram

	.bss (NOLOAD) : ALIGN(0x40)
	{
		__bss_start = .;
		*(.bss);
	} > ram

	__bss_end = ALIGN(0x40);

	.stack (NOLOAD) : ALIGN(0x40)
	{
		__stack_start = .;
		. += 0x400;
		__stack_end = ALIGN(0x40);
	} > ram

	/DISCARD/ :
	{
		*(.ARM.*);
	}
}
