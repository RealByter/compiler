	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	subq $76, %rsp
	movl $10, -4(%rbp)
	movl $3, -8(%rbp)
	movl -8(%rbp), %r11d
	imull $4, %r11d
	movl %r11d, -8(%rbp)
	movl $2, -12(%rbp)
	movl -8(%rbp), %r10d
	addl %r10d, -12(%rbp)
	movl -12(%rbp), %r10d
	movl %r10d, -16(%rbp)
	negl -16(%rbp)
	movl -16(%rbp), %eax
	cdq
	movl $2, %r10d
	idivl %r10d
	movl %eax, -20(%rbp)
	movl -20(%rbp), %eax
	cdq
	movl $7, %r10d
	idivl %r10d
	movl %edx, -24(%rbp)
	cmpl $0, -24(%rbp)
	movl $0, -28(%rbp)
	sete -28(%rbp)
	cmpl $0, -28(%rbp)
	movl $0, -32(%rbp)
	sete -32(%rbp)
	cmpl $1, -32(%rbp)
	movl $0, -36(%rbp)
	setne -36(%rbp)
	cmpl $0, -36(%rbp)
	je .Llabel_false.2
	movl $3, -40(%rbp)
	movl -40(%rbp), %r11d
	imull $4, %r11d
	movl %r11d, -40(%rbp)
	movl $2, -44(%rbp)
	movl -40(%rbp), %r10d
	addl %r10d, -44(%rbp)
	cmpl $10, -44(%rbp)
	movl $0, -48(%rbp)
	setg -48(%rbp)
	cmpl $0, -48(%rbp)
	je .Llabel_false.2
	movl $1, -52(%rbp)
	jmp .Llabel_and_end.3
.Llabel_false.2:
	movl $0, -52(%rbp)
.Llabel_and_end.3:
	cmpl $0, -52(%rbp)
	jne .Llabel_true.0
	movl $5, -56(%rbp)
	subl $3, -56(%rbp)
	cmpl $1, -56(%rbp)
	movl $0, -60(%rbp)
	setle -60(%rbp)
	cmpl $0, -60(%rbp)
	jne .Llabel_true.0
	movl $0, -64(%rbp)
	jmp .Llabel_or_end.1
.Llabel_true.0:
	movl $1, -64(%rbp)
.Llabel_or_end.1:
	movl -64(%rbp), %r10d
	movl %r10d, -68(%rbp)
	notl -68(%rbp)
	movl -68(%rbp), %r10d
	movl %r10d, -72(%rbp)
	notl -72(%rbp)
	movl -72(%rbp), %r10d
	movl %r10d, -76(%rbp)
	movl $20, -4(%rbp)
	movl -76(%rbp), %eax
	movq %rbp, %rsp
	popq %rbp
	ret
	movl $0, %eax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
