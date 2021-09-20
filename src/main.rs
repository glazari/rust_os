#![no_std] // don't link the Rust standard Library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world");

    rust_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    rust_os::hlt_loop();
}

// ----------------------------------------------------------------------
// Panic Functionality
// ----------------------------------------------------------------------

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trival_assertion() {
    assert_eq!(1, 1);
}
