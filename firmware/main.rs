#![no_std]
#![no_main]

mod mmio;
mod uart8250;
mod print;
mod devices;
mod serial;
mod sd;

use crate::mmio::*;

core::arch::global_asm!(include_str!("entry.S"));

unsafe extern "C" {
    static __dtb: usize;
}

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> !
{
    println!("[PANIC]");

    loop { core::hint::spin_loop(); }
}

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

        //match boot_mode {
        //    0 => sd::load(ddr2::BASE_ADDR);
        //    1 => serial::load(ddr2::BASE_ADDR);
        //    _ => panic!("MSEL");
        //}
        
        match boot_mode {
            0 => println!("SD"),
            1 => println!("SERIAL"),
            _ => panic!("MSEL"),
        }

        offwego(ddr2::BASE_ADDR, __dtb);
    }
}
