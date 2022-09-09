use pc_keyboard::{Keyboard, layouts, ScancodeSet1, DecodedKey, KeyCode, ScancodeSet};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::gdt;
use crate::print;
use crate::println;
use crate::vga_buffer::backspace;
use pic8259::ChainedPics;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);      // Switch to this stack, rather than continuing stackoverflow
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);  // Set timer handler func
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
// Handler function for normal debugger

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // use x86_64::instructions::interrupts::int3(); to trigger
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
// ----- Hardware interrupts
// all should notify EOI
extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    // Handler function for the hardware timer interrupt
    // print!(".");    // print out every ?? interval
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());     // must notify EOI
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) {
    
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, pc_keyboard::HandleControl::Ignore));
    }
    
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60); // communicate with the controller
    // the keyboard controller won’t send another interrupt 
    // until we have read the so-called scancode of the pressed key.
    let scancode: u8 = unsafe {port.read()};
    // Decode input key
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            // Match backspace event
            match ScancodeSet1::map_scancode(scancode).unwrap() {
                KeyCode::Backspace => {
                    // print!("backspace ");
                    backspace();
                }
                KeyCode::Escape => {
                }
                _ => {
                    match key {
                        DecodedKey::Unicode(character) => {
                            print!("{}", character)
                        }
                        // DecodedKey::RawKey(key) => print!("raw"),
                        DecodedKey::RawKey(key) => print!("{:?}", key), // not typical characters
                        // _ => {}
                    }
                }
            }
        }
    }
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());     // must notify EOI
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = 
    spin::Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,   // keyboard is interrupt 33
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        self as usize
    }
}