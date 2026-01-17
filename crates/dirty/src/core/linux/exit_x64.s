.global exit
exit:
	movq %rdi, %rax
	movq $60, %rax
	syscall
