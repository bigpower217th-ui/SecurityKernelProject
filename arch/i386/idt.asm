bits 32

extern irq_handler
extern exception_handler

%macro ISR_NOERRCODE 1
   global isr%1
    isr%1:
        push byte 0
	push byte %1
	jmp isr_common_stub
%endmacro

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 8
ISR_NOERRCODE 13
ISR_NOERRCODE 14

%macro IRQ 2
  global irq%1
  irq%1:
   push byte 0
  push byte %2
  jmp irq_common_stub
%endmacro

IRQ 0, 32
IRQ 1, 33
IRQ 9, 41

isr_common_stub;
   pusha
  mov ax, ds
  push eax
  mov ax, 0x10
  mov dx, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

call exception_handler

  pop eax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  popa
  add esp, 8
  iret

irq_common_stub:
   pusha
   mov ax, ds
   push eax
   mov ax, 0x10
   mov ds, ax
   mov es, ax
   mov fs, ax
   mov gs, ax

  call irq_handler

  pop ebx
 mov ds, bx
 mov es, bx
 mov fs, bx
 mov gs, bx
popa
add esp, 8
iret
global idt_flush
idt_flush:
    mov eax, [esp + 4]  
    lidt [eax]          
    ret
