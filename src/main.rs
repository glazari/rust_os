#![no_std] // don't link the Rust standard Library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
