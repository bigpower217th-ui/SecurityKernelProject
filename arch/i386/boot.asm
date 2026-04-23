[BITS 32]
section .multiboot
    align 4
   dd 0x1BADB002
   dd 0x00
   dd - (0x1BADB002 + 002)

section .text
global _start
extern kmain

_start:
  mov esp, stack_space
  call kmain
  hlt

section .bss 
resb 8192
stack_space:
