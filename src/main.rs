#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
mod interrupts;

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


fn init() {
    interrupts::init_idt(); // initialize idt
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    
    

    // trigger a stack overflow
    



    loop {}
}

