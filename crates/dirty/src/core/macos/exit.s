.global exit
exit:
	mov x16, #1
	svc 0x80
