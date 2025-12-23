#![allow(dead_code)]

#[inline(always)]
pub(crate) unsafe fn ioread8(addr: usize) -> u8
{
    unsafe { return core::ptr::read_volatile(addr as *const u8); }
}

#[inline(always)]
pub(crate) unsafe fn iowrite8(addr: usize, val: u8)
{
    unsafe { core::ptr::write_volatile(addr as *mut u8, val); }
}

#[inline(always)]
pub(crate) unsafe fn ioread32(addr: usize) -> u32
{
    unsafe { return core::ptr::read_volatile(addr as *const u32); }
}

#[inline(always)]
pub(crate) unsafe fn iowrite32(addr: usize, val: u32)
{
    unsafe { core::ptr::write_volatile(addr as *mut u32, val); }
}

//
// UART 16550
//
pub(crate) mod uart {
    pub(crate) const BASE_ADDR: usize = 0x4A00_0000;

    pub(crate) const CLK_FREQ: u32 = 100_000_000;
    pub(crate) const BAUDRATE: u32 = 115200;

    pub(crate) const RBR: usize = BASE_ADDR + 0x00;
    pub(crate) const THR: usize = BASE_ADDR + 0x00;
    pub(crate) const DLL: usize = BASE_ADDR + 0x00;
    pub(crate) const IER: usize = BASE_ADDR + 0x04;
    pub(crate) const DLM: usize = BASE_ADDR + 0x04;
    pub(crate) const FCR: usize = BASE_ADDR + 0x08;
    pub(crate) const LCR: usize = BASE_ADDR + 0x0C;
    pub(crate) const MCR: usize = BASE_ADDR + 0x10;
    pub(crate) const LSR: usize = BASE_ADDR + 0x14;
    pub(crate) const MSR: usize = BASE_ADDR + 0x18;
    pub(crate) const SCR: usize = BASE_ADDR + 0x1C;

    pub(crate) const LCR_DLAB: u32 = 1 << 7;
    pub(crate) const LCR_8N1: u32 = 0x03;
    pub(crate) const LSR_DR: u32 = 1 << 0;
    pub(crate) const LSR_THRE: u32 = 1 << 5;
    pub(crate) const FCR_ENABLE_FIFO: u32 = 1 << 0;
    pub(crate) const FCR_CLEAR_RX: u32 = 1 << 1;
    pub(crate) const FCR_CLEAR_TX: u32 = 1 << 2;
    pub(crate) const MCR_DTR: u32 = 1 << 0;
    pub(crate) const MCR_RTS: u32 = 1 << 1;
}

//
// GPIO
//
pub(crate) mod gpio {
    pub(crate) const SWITCHES_BASE_ADDR: usize = 0x4000_0000;
    pub(crate) const BUTTONS_BASE_ADDR: usize = 0x4100_0000;
    pub(crate) const LEDS_BASE_ADDR: usize = 0x4200_0000;

    pub(crate) const GIER_OFFSET: usize = 0x11C;
    pub(crate) const IP_IER_OFFSET: usize = 0x128;
    pub(crate) const IP_ISR_OFFSET: usize = 0x120;

    pub(crate) const GIER_ENABLE: u32 = 1 << 31;
    pub(crate) const IER_CH1_ENABLE: u32 = 1 << 0;
    pub(crate) const IER_CH2_ENABLE: u32 = 1 << 1;

    pub(crate) const SWITCHES_IPISR: usize = SWITCHES_BASE_ADDR + IP_ISR_OFFSET;
    pub(crate) const BUTTONS_IPISR: usize = BUTTONS_BASE_ADDR + IP_ISR_OFFSET;
}

//
// CLINT
//
pub(crate) mod clint {
    pub(crate) const BASE_ADDR: usize = 0x3000_0000;

    pub(crate) const MSIP: usize = BASE_ADDR;
    pub(crate) const MTIMECMP: usize = BASE_ADDR + 0x4000;
    pub(crate) const MTIME: usize = BASE_ADDR + 0xBFF8;
}

//
// PLIC
//
pub(crate) mod plic {
    pub(crate) const BASE_ADDR: usize = 0x3A00_0000;

    pub(crate) const PENDING: usize = BASE_ADDR + 0x1000;
    pub(crate) const M_PRIORITY: usize = BASE_ADDR + 0x04;
    pub(crate) const M_ENABLE: usize = BASE_ADDR + 0x2000;
    pub(crate) const M_THRESHOLD: usize = BASE_ADDR + 0x200000;
    pub(crate) const M_CLAIM: usize = BASE_ADDR + 0x200004;

    pub(crate) const MAX_SOURCES: u32 = 32;

    pub(crate) const UART_ID: u32 = 2;
    pub(crate) const AUDIO_ID: u32 = 3;
    pub(crate) const AUDIO_DMA_ID: u32 = 4;
    pub(crate) const SDC_DATA_ID: u32 = 5;
    pub(crate) const SDC_CMD_ID: u32 = 6;
    pub(crate) const GPIO_SWITCHES_ID: u32 = 7;
    pub(crate) const GPIO_BUTTONS_ID: usize = 8;
}

//
// DDR2 SDRAM
//
pub(crate) mod ddr2 {
    pub(crate) const BASE_ADDR: usize = 0x8000_0000;

    pub(crate) const SIZE: u32 = 128 * 1024 * 1024;
}

//
// FRAMEBUFFER
//
pub(crate) mod fb {
    pub(crate) const BASE_ADDR: usize = 0x2B00_0000;

    pub(crate) const WIDTH: u32 = 640;
    pub(crate) const HEIGHT: u32 = 480;
    pub(crate) const BPP: u32 = 1;
}

//
// AUDIO PWM
//
pub(crate) mod audio {
    pub(crate) const BASE_ADDR: usize = 0x6000_0000;

    pub(crate) const CLK_DIV: usize = BASE_ADDR + 0x00;
    pub(crate) const IRQ_STATE: usize = BASE_ADDR + 0x04;
    pub(crate) const CHIP_STATE: usize = BASE_ADDR + 0x08;
    pub(crate) const IRQ_SIG: usize = BASE_ADDR + 0x0C;

    pub(crate) const DMA_MM2S_DMACR: usize = BASE_ADDR + 0x10000;
    pub(crate) const DMA_MM2S_DMASR: usize = BASE_ADDR + 0x10004;
    pub(crate) const DMA_MM2S_SA: usize = BASE_ADDR + 0x10018;
    pub(crate) const DMA_MM2S_SA_MSB: usize = BASE_ADDR + 0x1001C;
    pub(crate) const DMA_MM2S_LENGTH: usize = BASE_ADDR + 0x10028;

    pub(crate) const DMA_RST_BIT: u32 = 1 << 2;
    pub(crate) const DMA_HALTED_BIT: u32 = 1 << 0;
    pub(crate) const DMA_RS_BIT: u32 = 1 << 0;
    pub(crate) const DMA_IOC_IRQEN: u32 = 1 << 12;
    pub(crate) const DMA_ERR_IRQEN: u32 = 1 << 14;
}

//
// PMC
//
pub(crate) mod pmc {
    pub(crate) const BASE_ADDR: usize = 0x1000_0000;

    pub(crate) const CONTROL: usize = BASE_ADDR + 0x00;

    pub(crate) const POWEROFF: u32 = 0xB2;
    pub(crate) const REBOOT: u32 = 0xF7;
}
