#include "kernel.h"

void kprint(char *message) {
    
    unsigned char *vga_mem = (unsigned char *)0xB8000;
    
    
    while (*message != '\0') {
        *vga_mem = *message;     
        vga_mem++;               
        *vga_mem = 0x0F;         
         vga_mem++;                
        message++;               
    }
}
