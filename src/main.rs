#![no_std]
#![no_main]

//cargo build --target thumbv7em-none-eabihf
//can't cargo run because of no_std and no_main

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        //gee whiz i sure do hope it doesnt break

    }

}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        //entry point of the kernel 

    }
}