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

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    john_os::test_panic_handler(info)
}
