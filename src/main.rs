#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
pub mod interrupts;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world\n");
    println!("hello world\n");
    loop {}
}

