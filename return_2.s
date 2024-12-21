	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	subq $24, %rsp
	movl $3, -4(%rbp)
	movl -4(%rbp), %r11d
	imull $4, %r11d
	movl %r11d, -4(%rbp)
	movl $2, -8(%rbp)
	movl -4(%rbp), %r10d
	addl %r10d, -8(%rbp)
	movl -8(%rbp), %r10d
	movl %r10d, -12(%rbp)
	negl -12(%rbp)
	movl -12(%rbp), %eax
	cdq
	movl $2, %r10d
	idivl %r10d
	movl %eax, -16(%rbp)
	movl -16(%rbp), %eax
	cdq
	movl $6, %r10d
	idivl %r10d
	movl %edx, -20(%rbp)
	movl -20(%rbp), %r10d
	movl %r10d, -24(%rbp)
	notl -24(%rbp)
	movl -24(%rbp), %eax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
