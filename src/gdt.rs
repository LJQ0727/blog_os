use lazy_static::lazy_static;
use x86_64::{structures::{tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}}, VirtAddr, registers::segmentation::Segment, instructions::tables::load_tss};

// The Global Descriptor Table (GDT) is a relic that was used for memory segmentation before paging became the de facto standard. However, it is still needed in 64-bit mode 
// for various things, such as kernel/user mode configuration or TSS loading.

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector})
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    // The x86_64 architecture is able to switch to a predefined, 
    // known-good stack when an exception occurs. This switch happens at hardware level, so it can be performed before the CPU pushes the exception stack frame.

    // So that the interrupt stack frame is pushed to a safe, reserved memory address
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe {&STACK});
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::CS;
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}