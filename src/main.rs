// freestanding rust binary

#![no_std] // Disabling the implicit inclusion of the standard library
#![no_main] // We cant use the regular main function as a start point for our kernel.

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() ->! {
    
    println!("Hello from: {}!", "Crab OS");
    panic!("An example of a panic message!");
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    error_nl!("{}", info);
    loop{}
}
