#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
mod interrupts;

use core::panic::PanicInfo;

mod vga_buffer;
mod gdt;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}


fn init() {
    // initialize gdt, idt, receiver for interrupts
    gdt::init();
    interrupts::init_idt(); // initialize idt
    unsafe {interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    
    
    


    hlt_loop();
}

pub fn hlt_loop() -> ! {
    // replacement for the loop {}
    loop {
        x86_64::instructions::hlt();    // Halts the cpu until the next instruction arrives
    }
}