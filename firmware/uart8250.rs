use crate::mmio::*;

pub(crate) unsafe fn init(baudrate: u32, clk_hz: u32)
{
    unsafe {
        let divisor = (clk_hz / (16 * baudrate)) as u16;

        iowrite32(uart::LCR, uart::LCR_DLAB);
        iowrite32(uart::DLL, (divisor & 0xFF) as u32);
        iowrite32(uart::DLM, ((divisor >> 8) & 0xFF) as u32);
        iowrite32(uart::LCR, uart::LCR_8N1);
        iowrite32(uart::FCR, uart::FCR_ENABLE_FIFO | uart::FCR_CLEAR_RX | uart::FCR_CLEAR_TX);
        iowrite32(uart::MCR, uart::MCR_DTR | uart::MCR_RTS);

        /* disable interrupts */
        iowrite32(uart::IER, 0);
    }
}

pub(crate) unsafe fn putc(c: u8)
{
    unsafe {
        while (ioread32(uart::LSR) & uart::LSR_THRE) == 0 {
            core::hint::spin_loop();
        }

        iowrite32(uart::THR, c as u32);
    }
}

pub(crate) unsafe fn getc() -> u8
{
    unsafe {
        while (ioread32(uart::LSR) & uart::LSR_DR) == 0 {
            core::hint::spin_loop();
        }

        return ioread32(uart::RBR) as u8;
    }
}
