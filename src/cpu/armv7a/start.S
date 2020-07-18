	.global _start
	.text

_start:
	ldr	sp, =__stack_end

	//  clear .bss section
	ldr	r0, =__bss_start
	ldr	r1, =__bss_end
	mov	r2, #0
1:
	cmp	r0, r1
	stmiane	r0!, {r2}
	bne	1b

	b	_main
