#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as runtime;
extern crate stm32f7;

use core::panic::PanicInfo;
// use cortex_m::asm;

use cortex_m_log::{print, println, d_print, d_println};
use cortex_m_log::printer::Dummy;

#[no_mangle]
fn main() -> ! {   

    let mut log = Dummy::new();
    println!(log, "Some print with newline!");
    //Debug version of print that resolves into nothing in release mode
    //Note that you must import print macro for it to work
    d_print!(log, "Print stuff: {}", "stuff");
    //Note that you must import println macro for it to work
    d_println!(log, "Print stuff: {} and also newline", "stuff");

    loop {

        for _i in 0..1000000 {
            //  asm::nop()
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
