	.global _start
	.text

_start:
	ldr	sp, =__stack_end
	b	_main
