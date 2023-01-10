// freestanding rust binary

#![no_std] // Disabling the implicit inclusion of the standard library
#![no_main] // We cant use the regular main function as a start point for our kernel.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello from Crab OS!";

#[no_mangle]
pub extern "C" fn _start() ->! {
    let vga_buffer = 0xb8000 as *mut u8 ;
    // A very minamialistic/simple version of our kernel!
    // Simple Hello world writing into the VGA buffer.
    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe { // Needed an unsafe block since we are updating memory location with our raw pointers,
                 // which violates the memory safety of rust :(
        // TODO: Create a VGA buffer abstraction, in order to minimize the usage of unsafe blocks.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

 loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) ->! {
    loop{}
}
