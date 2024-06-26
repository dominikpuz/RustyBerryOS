INCLUDE kernel_virt_addr_space_size.ld;

PAGE_SIZE = 64K;
PAGE_MASK = PAGE_SIZE - 1;

/* The kernel's virtual address range will be:
 *
 * [END_ADDRESS_INCLUSIVE, START_ADDRESS]
 * [u64::MAX             , (u64::MAX - __kernel_virt_addr_space_size) + 1]
 */
__kernel_virt_start_addr = ((0xffffffffffffffff - __kernel_virt_addr_space_size) + 1);

__rpi_phys_dram_start_addr = 0;

/* The physical address at which the the kernel binary will be loaded by the Raspberry's firmware */
__rpi_phys_binary_load_addr = 0x80000;


ENTRY(__rpi_phys_binary_load_addr)

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
    segment_boot_core_stack PT_LOAD FLAGS(6);
}

SECTIONS
{
     . =  __kernel_virt_start_addr;
     ASSERT((. & PAGE_MASK) == 0, "Start of address space is not page aligned")

    /***********************************************************************************************
    * Code + RO Data + Global Offset Table
    ***********************************************************************************************/
     __code_start = .;
    .text : AT(__rpi_phys_binary_load_addr)
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* statics in Rust read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } :segment_code

    .rodata : ALIGN(8) { *(.rodata*) } :segment_code

    . = ALIGN(PAGE_SIZE);
    __code_end_exclusive = .;

    /***********************************************************************************************
    * Data + BSS
    ***********************************************************************************************/
    __data_start = .;
    .data : { *(.data*) } :segment_data

    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data

    . = ALIGN(PAGE_SIZE);
    __data_end_exclusive = .;

    /***********************************************************************************************
    * MMIO Remap Reserved
    ***********************************************************************************************/
    __mmio_remap_start = .;
    . += 8 * 1024 * 1024;
    __mmio_remap_end_exclusive = .;

    ASSERT((. & PAGE_MASK) == 0, "MMIO remap reservation is not page aligned")

    /***********************************************************************************************
    * Guard Page
    ***********************************************************************************************/
    . += PAGE_SIZE;

    /***********************************************************************************************
    * Boot Core Stack
    ***********************************************************************************************/
    .boot_core_stack (NOLOAD) : AT(__rpi_phys_dram_start_addr)
    {
        __boot_core_stack_start = .;         /*   ^             */
                                             /*   | stack       */
        . += __rpi_phys_binary_load_addr;    /*   | growth      */
                                             /*   | direction   */
        __boot_core_stack_end_exclusive = .; /*   |             */
    } :segment_boot_core_stack

    ASSERT((. & PAGE_MASK) == 0, "End of boot core stack is not page aligned")

    /***********************************************************************************************
    * Misc
    ***********************************************************************************************/
    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }
}