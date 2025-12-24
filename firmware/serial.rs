use crate::mmio::*;
use crate::error::FirmwareError;
use crate::uart8250::{getc};

pub unsafe fn load(addr: usize) -> Result<(), FirmwareError>
{
    Ok(())
}
