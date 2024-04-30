
// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

// Load the address of a symbol into a register, absolute.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_ABS register, symbol
	movz	\register, #:abs_g2:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
.endm


// ---------------
// Public code 
// ---------------
.section .text._start

// ----------------
// fn _start()
// ----------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x0, MPIDR_EL1
	and	x0, x0, {CONST_CORE_ID_MASK} // x0 = x0 & MASK
	ldr	x1, BOOT_CORE_ID      // provided by bsp/raspberrypi/cpu.rs
	cmp	x0, x1
	b.ne	.parking_loop // if x0 != x1, jump to .parking_loop


	// Initialize DRAM.
	ADR_ABS	x0, __bss_start
	ADR_ABS x1, __bss_end_exclusive

.bss_init_loop:
    cmp	x0, x1
    b.eq	.L_relocate_binary
    stp    xzr, xzr, [x0], #16
    b	.bss_init_loop

	// Next, relocate the binary.
.L_relocate_binary:
	ADR_REL	x0, __binary_nonzero_start         // The address the binary got loaded to.
	ADR_ABS	x1, __binary_nonzero_start         // The address the binary was linked to.
	ADR_ABS	x2, __binary_nonzero_end_exclusive

.L_copy_loop:
	ldr	x3, [x0], #8
	str	x3, [x1], #8
	cmp	x1, x2
	b.lo	.L_copy_loop


    ADR_ABS x0, __boot_core_stack_end_exclusive
    mov sp, x0 // Set the stack pointer

    // Read the CPU's timer counter frequency and store it in ARCH_TIMER_COUNTER_FREQUENCY.
    // Abort if the frequency read back as 0.
    ADR_REL	x1, ARCH_TIMER_COUNTER_FREQUENCY // provided by aarch64/time.rs
    mrs	x2, CNTFRQ_EL0
    cmp	x2, xzr
    b.eq	.parking_loop
    str	w2, [x1]

    // Call the Rust code
    ADR_ABS x1, _start_rust
    br x1

.parking_loop:
  	wfe // inf while wait for event
    b	.parking_loop

.size _start, . - _start
.type	_start, function
.global	_start
