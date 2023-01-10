// freestanding rust binary

#![no_std] // Disabling the implicit inclusion of the standard library
#![no_main] // We cant use the regular main function as a start point for our kernel.

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() ->! {
 loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) ->! {
    loop{}
}
