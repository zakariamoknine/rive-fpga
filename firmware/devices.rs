use crate::mmio::*;

unsafe fn init_irqchips()
{
    unsafe {
        for i in 0..plic::MAX_SOURCES {
            iowrite32(plic::M_PRIORITY + ((4 * i) as usize), 0);
        }

        iowrite32(plic::M_THRESHOLD, 0);

        // Disable both contexts (S and M modes)
        iowrite32(plic::M_ENABLE, 0);
        iowrite32(plic::M_ENABLE + 0x04, 0);
    }
}

unsafe fn init_audio()
{
    unsafe {
        // Disable the chip entirely
        iowrite32(audio::CLK_DIV, 2268);
        iowrite32(audio::IRQ_STATE, 0);
        iowrite32(audio::CHIP_STATE, 0);

        // Perform a full reset on AXI DMA
        iowrite32(audio::DMA_MM2S_DMACR, audio::DMA_RST_BIT);

        while (ioread32(audio::DMA_MM2S_DMACR) & audio::DMA_RST_BIT) != 0 {
            core::hint::spin_loop();
        }
    }
}

unsafe fn init_pmc()
{
    unsafe {
        iowrite32(pmc::CONTROL, 0);
    }
}

pub(crate) unsafe fn init()
{
    unsafe {
        init_irqchips();
        init_audio();
        init_pmc();
    }
}

pub(crate) unsafe fn draw()
{
    let fb_ptr = fb::BASE_ADDR as *mut u8;
    let width  = fb::WIDTH as usize;
    let height = fb::HEIGHT as usize;

    for y in 0..height {
        for x in 0..width {
            let r = (x * 7) / (width - 1);
            let g = (y * 7) / (height - 1);
            let b = 0;

            let color = ((r as u8) << 5) | ((g as u8) << 2) | (b as u8);

            unsafe { fb_ptr.add(y * width + x).write_volatile(color); }
        }
    }
}

pub(crate) unsafe fn msel() -> u32
{
    unsafe { ioread32(gpio::SWITCHES_BASE_ADDR) & 0x01 }
}
