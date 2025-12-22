#![no_std]
#![no_main]

mod mmio;
use crate::mmio::*;

core::arch::global_asm!(include_str!("entry.S"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !
{
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn start() -> !
{
    let fb_ptr = fb::BASE_ADDR as *mut u8;

    for i in 0..(fb::WIDTH * fb::HEIGHT) {
        unsafe {
            fb_ptr.add(i as usize).write_volatile((i ^ 0x10) as u8);
        }
    }

    loop {}
}
