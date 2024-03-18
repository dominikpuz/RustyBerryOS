// ---------------
// Public code 
// ---------------
.section .text._start

// fn _start()
_start:
    // infinite loop

.parking_loop:
  	wfe // inf while wait for event
    b	.parking_loop

.size _start, . - _start
.type	_start, function
.global	_start
