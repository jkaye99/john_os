#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(john_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use john_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, John!");

    john_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    john_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    john_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    john_os::test_panic_handler(info)
}
