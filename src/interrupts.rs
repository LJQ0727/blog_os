use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::gdt;
use crate::println;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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