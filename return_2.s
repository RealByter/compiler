	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	subq $112, %rsp
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
	cmpl $0, -4(%rbp)
	je .Llabel_if_end.5
	cmpl $15, -4(%rbp)
	movl $0, -80(%rbp)
	setg -80(%rbp)
	cmpl $0, -80(%rbp)
	je .Llabel_false.6
	movl $1, -84(%rbp)
	jmp .Llabel_if_end.7
.Llabel_false.6:
	movl $2, -84(%rbp)
	movl $4, -84(%rbp)
.Llabel_if_end.7:
	movl $5, -88(%rbp)
.Llabel_if_end.5:
	cmpl $1, -84(%rbp)
	movl $0, -92(%rbp)
	sete -92(%rbp)
	cmpl $0, -92(%rbp)
	je .Llabel_false.8
	movl -84(%rbp), %r10d
	movl %r10d, -96(%rbp)
	movl -96(%rbp), %r11d
	imull $2, %r11d
	movl %r11d, -96(%rbp)
	movl -96(%rbp), %r10d
	movl %r10d, -100(%rbp)
	jmp .Llabel_cond_end.9
.Llabel_false.8:
	movl -84(%rbp), %r10d
	movl %r10d, -104(%rbp)
	movl -104(%rbp), %r11d
	imull $3, %r11d
	movl %r11d, -104(%rbp)
	movl -104(%rbp), %r10d
	movl %r10d, -100(%rbp)
.Llabel_cond_end.9:
	movl -100(%rbp), %r10d
	movl %r10d, -108(%rbp)
	movl -4(%rbp), %r10d
	movl %r10d, -112(%rbp)
	movl -112(%rbp), %r11d
	imull $20, %r11d
	movl %r11d, -112(%rbp)
	movl -112(%rbp), %r10d
	movl %r10d, -4(%rbp)
	movl -4(%rbp), %eax
	movq %rbp, %rsp
	popq %rbp
	ret
	movl $0, %eax
	movq %rbp, %rsp
	popq %rbp
	ret
.section .note.GNU-stack,"",@progbits
