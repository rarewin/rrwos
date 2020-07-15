MEMORY
{
	rom (rx) : ORIGIN = 0x00000000, LENGTH = 16M
	ram (rwx): ORIGIN = 0x40000000, LENGTH = 128M
}

SECTIONS
{
	.text : ALIGN(0x40)
	{
		*(.text);
	} > rom

	.rodata : ALIGN(0x40)
	{
		*(.rodata);
	} > rom

	.data : ALIGN(0x40)
	{
		*(.data);
	} > ram AT> rom

	.bss : ALIGN(0x40)
	{
		*(.bss);
	} > ram AT> rom

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
