	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	subq $68, %rsp
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
	movl $7, %r10d
	idivl %r10d
	movl %edx, -20(%rbp)
	cmpl $0, -20(%rbp)
	movl $0, -24(%rbp)
	sete -24(%rbp)
	cmpl $0, -24(%rbp)
	movl $0, -28(%rbp)
	sete -28(%rbp)
	cmpl $1, -28(%rbp)
	movl $0, -32(%rbp)
	setne -32(%rbp)
	cmpl $0, -32(%rbp)
	je .Llabel_false.2
	movl $3, -36(%rbp)
	movl -36(%rbp), %r11d
	imull $4, %r11d
	movl %r11d, -36(%rbp)
	movl $2, -40(%rbp)
	movl -36(%rbp), %r10d
	addl %r10d, -40(%rbp)
	cmpl $10, -40(%rbp)
	movl $0, -44(%rbp)
	setg -44(%rbp)
	cmpl $0, -44(%rbp)
	je .Llabel_false.2
	movl $1, -48(%rbp)
	jmp .Llabel_and_end.3
.Llabel_false.2:
	movl $0, -48(%rbp)
.Llabel_and_end.3:
	cmpl $0, -48(%rbp)
	jne .Llabel_true.0
	movl $5, -52(%rbp)
	subl $3, -52(%rbp)
	cmpl $1, -52(%rbp)
	movl $0, -56(%rbp)
	setle -56(%rbp)
	cmpl $0, -56(%rbp)
	jne .Llabel_true.0
	movl $0, -60(%rbp)
	jmp .Llabel_or_end.1
.Llabel_true.0:
	movl $1, -60(%rbp)
.Llabel_or_end.1:
	movl -60(%rbp), %r10d
	movl %r10d, -64(%rbp)
	notl -64(%rbp)
	movl -64(%rbp), %r10d
	movl %r10d, -68(%rbp)
	notl -68(%rbp)
	movl -68(%rbp), %eax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
