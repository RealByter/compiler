	.globl return3
return3:
	pushq %rbp
	movq %rsp, %rbp
	subq $4, %rsp
	movl $3, -4(%rbp)
	movl $3, %eax
	movq %rbp, %rsp
	popq %rbp
	ret
	movl $0, %eax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
