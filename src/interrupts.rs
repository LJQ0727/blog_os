use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// pub fn init_idt() {
//     let mut idt = InterruptDescriptorTable::new();
//     idt.breakpoint.set_handler_fn(breakpoint_handler);
// }

// extern "x86_interrupts" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
//     println!("Exception: Breakpoint\n{:#?}", stack_frame);
// }