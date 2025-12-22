#![allow(dead_code)]

/*
 * UART 16550
 */
pub mod uart {
    pub const BASE_ADDR: usize = 0x4A00_0000;

    pub const CLK_FREQ: u32 = 100_000_000;
    pub const BAUDRATE: u32 = 115200;

    pub const RBR: usize = 0x00;
    pub const THR: usize = 0x00;
    pub const DLL: usize = 0x00;
    pub const IER: usize = 0x04;
    pub const DLM: usize = 0x04;
    pub const FCR: usize = 0x08;
    pub const LCR: usize = 0x0C;
    pub const MCR: usize = 0x10;
    pub const LSR: usize = 0x14;
    pub const MSR: usize = 0x18;
    pub const SCR: usize = 0x1C;

    pub const LCR_DLAB: u32 = 1 << 7;
    pub const LCR_8N1: u32 = 0x03;
    pub const LSR_DR: u32 = 1 << 0;
    pub const LSR_THRE: u32 = 1 << 5;
    pub const FCR_ENABLE_FIFO: u32 = 1 << 0;
    pub const FCR_CLEAR_RX: u32 = 1 << 1;
    pub const FCR_CLEAR_TX: u32 = 1 << 2;
    pub const MCR_DTR: u32 = 1 << 0;
    pub const MCR_RTS: u32 = 1 << 1;
}

/*
 * GPIO
 */
pub mod gpio {
    pub const LEDS_BASE_ADDR: usize = 0x4000_0000;
    pub const SWITCHES_BASE_ADDR: usize = 0x4001_0000;
    pub const BUTTONS_BASE_ADDR: usize = 0x4002_0000;

    pub const GIER_OFFSET: usize = 0x11C;
    pub const IP_IER_OFFSET: usize = 0x128;
    pub const IP_ISR_OFFSET: usize = 0x120;

    pub const GIER_ENABLE: u32 = 1 << 31;
    pub const IER_CH1_ENABLE: u32 = 1 << 0;
    pub const IER_CH2_ENABLE: u32 = 1 << 1;

    pub const SWITCHES_IPISR: usize = SWITCHES_BASE_ADDR + IP_ISR_OFFSET;
    pub const BUTTONS_IPISR: usize = BUTTONS_BASE_ADDR + IP_ISR_OFFSET;
}

/*
 * CLINT
 */
pub mod clint {
    pub const BASE_ADDR: usize = 0x3000_0000;

    pub const MSIP: usize = BASE_ADDR;
    pub const MTIMECMP: usize = BASE_ADDR + 0x4000;
    pub const MTIME: usize = BASE_ADDR + 0xBFF8;
}

/*
 * PLIC
 */
pub mod plic {
    pub const BASE_ADDR: usize = 0x3A00_0000;

    pub const PENDING: usize = BASE_ADDR + 0x1000;
    pub const M_PRIORITY: usize = BASE_ADDR + 0x04;
    pub const M_ENABLE: usize = BASE_ADDR + 0x2000;
    pub const M_THRESHOLD: usize = BASE_ADDR + 0x200000;
    pub const M_CLAIM: usize = BASE_ADDR + 0x200004;

    pub const MAX_SOURCES: u32 = 32;

    pub const UART_ID: u32 = 2;
    pub const AUDIO_ID: u32 = 3;
    pub const AUDIO_DMA_ID: u32 = 4;
    pub const SDC_DATA_ID: u32 = 5;
    pub const SDC_CMD_ID: u32 = 6;
    pub const GPIO_SWITCHES_ID: u32 = 7;
    pub const GPIO_BUTTONS_ID: usize = 8;
}

/*
 * DDR2 SDRAM & Framebuffer
 */
pub mod ddr2 {
    pub const BASE_ADDR: usize = 0x8000_0000;

    pub const SIZE: u32 = 128 * 1024 * 1024;
}


pub mod fb {
    pub const BASE_ADDR: usize = 0x2B00_0000;

    pub const WIDTH: u32 = 640;
    pub const HEIGHT: u32 = 480;
    pub const BPP: u32 = 1;
}

/*
 * AUDIO PWM & DMA
 */
pub mod audio {
    pub const BASE_ADDR: usize = 0x6000_0000;

    pub const CLK_DIV: usize = BASE_ADDR + 0x00;
    pub const IRQ_STATE: usize = BASE_ADDR + 0x04;
    pub const CHIP_STATE: usize = BASE_ADDR + 0x08;
    pub const IRQ_SIG: usize = BASE_ADDR + 0x0C;

    pub const DMA_MM2S_DMACR: usize = BASE_ADDR + 0x10000;
    pub const DMA_MM2S_DMASR: usize = BASE_ADDR + 0x10004;
    pub const DMA_MM2S_SA: usize = BASE_ADDR + 0x10018;
    pub const DMA_MM2S_SA_MSB: usize = BASE_ADDR + 0x1001C;
    pub const DMA_MM2S_LENGTH: usize = BASE_ADDR + 0x10028;

    pub const DMA_RST_BIT: u32 = 1 << 2;
    pub const DMA_HALTED_BIT: u32 = 1 << 0;
    pub const DMA_RS_BIT: u32 = 1 << 0;
    pub const DMA_IOC_IRQEN: u32 = 1 << 12;
    pub const DMA_ERR_IRQEN: u32 = 1 << 14;
}

/*
 * PMC
 */
pub mod pmc {
    pub const BASE_ADDR: usize = 0x1000_0000;

    pub const CONTROL: usize = BASE_ADDR + 0x00;

    pub const POWEROFF: u32 = 0xB2;
    pub const REBOOT: u32 = 0xF7;
}
