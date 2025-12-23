#![no_std]
#![no_main]

mod mmio;
use crate::mmio::*;

mod uart8250;
mod print;

core::arch::global_asm!(include_str!("entry.S"));

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> !
{
    println!("[ PANIC ]");

    loop { core::hint::spin_loop(); }
}

unsafe fn fb_draw() {
    let fb_ptr = fb::BASE_ADDR as *mut u8;
    let width  = fb::WIDTH as usize;
    let height = fb::HEIGHT as usize;

    for i in 0..(width * height) {
        unsafe { fb_ptr.add(i).write_volatile((i ^ 0xFF) as u8); }
    }
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn start_firmware() -> !
{
    unsafe {
        // Initialize uart16550
        uart8250::init(uart::BAUDRATE, uart::CLK_FREQ);

        // Indicate we're in
        println!("Hello, Rive!");
        fb_draw();
    }

    loop {}
}
