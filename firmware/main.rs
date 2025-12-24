#![no_std]
#![no_main]

mod mmio;
mod error;
mod uart8250;
mod print;
mod devices;
mod serial;
mod sd;

use crate::mmio::*;

core::arch::global_asm!(include_str!("entry.S"));

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> !
{
    print!("[PANIC]: ");
    println!("{}", panic_info.message());

    // Wait 2 seconds then reboot the box
    println!("Rebooting..");
    unsafe {
        devices::udelay(2000000);
        devices::reboot();
    }
}

unsafe extern "C" {
    static __dtb: usize;
    static __dtb_end: usize;
}

// High enough to not overlap with the payload later, if it
// does, then the payload is a way too big
const DTB_BASE_ADDR: usize = ddr2::BASE_ADDR + 0x0700_0000;

unsafe fn offwego(addr: usize, dtb: usize) -> !
{
    unsafe {
        // fence.i to flush the i-cache
        // 5 nops to ensure the pipeline is flushed
        core::arch::asm!(
            "fence.i",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "mv   a1, {dtb}",
            "mv   a0, x0",
            "jr   {addr}",
            addr = in(reg) addr,
            dtb = in(reg) dtb,
            options(noreturn)
        );
    }
}

unsafe fn copy_dtb(addr: usize)
{
    unsafe {
        let start = &__dtb as *const usize as usize;
        let end = &__dtb_end as *const usize as usize;

        let mut i = 0;
        while (start + i) < end {
            iowrite8(addr + i, ioread8(start + i));
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn start_firmware() -> !
{
    unsafe {
        // Initialize uart16550
        uart8250::init(uart::BAUDRATE, uart::CLK_FREQ);

        // Initialize devices
        devices::init();

        // Indicate we're in
        println!("rive-fpga is booting..");
        devices::draw();

        // Read MSEL for boot mode
        let boot_mode = devices::msel();

        let result = match boot_mode {
            0 => sd::load(ddr2::BASE_ADDR),
            1 => serial::load(ddr2::BASE_ADDR),
            _ => panic!("INVALID MSEL CONFIGURATION"),
        };

        result.expect("PAYLOAD LOADING FAILED");

        copy_dtb(DTB_BASE_ADDR);

        offwego(ddr2::BASE_ADDR, DTB_BASE_ADDR);
    }
}
