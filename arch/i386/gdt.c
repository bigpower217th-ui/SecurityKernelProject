struct gdt_entry {
    unsigned short limit_low;
    unsigned short base_low;
    unsigned char base_middle;
    unsigned char acces;
    unsigned char granularity;
    unsigned char base_high;
} __attribute__((packed));

struct gdt_entry gdt[3];
void init_gdt() {
}
