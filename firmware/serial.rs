use crate::mmio::*;
use crate::error::FirmwareError;
const SERIAL_MAGIC: u32 = 0x52444C52; 
const MAX_SIZE: u32 = 128 * 1024 * 1024;

#[repr(C)]
pub struct SerialHeader { 
    pub magic : u32, 
    pub size : u32, 
} 
impl SerialHeader{ 
    pub fn parse(bytes : &[u8])-> Result<Self, FirmwareError> {
        if bytes.len() < core::mem::size_of::<SerialHeader>(){ 
            return Err(FirmwareError::UartSizeOverflow); 
        }
        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap()); 
        let size = u32::from_le_bytes(bytes[4..8].try_into().unwrap()); 
        Ok(Self{ 
            magic, 
            size, 
            entry_offset,
        })   
    }
    
    // pub unsafe fn load(addr: usize) -> Result<(), FirmwareError>
    // { 
    //    let mut adr = addr as *mut u8; 
    //    for _ in 0..size { 
    //     let byte = .read_byte(); 
    //     unsafe  { 
    //         core::ptr::write_volatile(addr, byte);
    //     } 
    //    }
    //  }
    pub fn validate(&self)-> Result<(), FirmwareError> { 
        if self.magic != SERIAL_MAGIC {
            return Err(FirmwareError::UartInvalidMagicNumber); 
        }
        if self.size ==0 || self.size > ddr2::SIZE { 
            return Err(FirmwareError::UartSizeOverflow);
        }
        if self.entry_offset >= self.size { 
            return Err(FirmwareError::UartEntryOffsetOverflow); 
        }
        Ok(())
    }

} 
pub fn read_header() -> Result<SerialHeader, FirmwareError> { 
        const HEADER_SIZE:usize = core::mem::size_of::<SerialHeader>(); 
        let mut buf = [u8; HEADER_SIZE]; 
        for i in 0..HEADER_SIZE { 
            buf[i] = getc(); 
        }
        let header = SerialHeader::parse(&buf); 
        header.validate()?; 
        Ok(header)
    } 
pub unsafe fn load_payload(base :usize ,size: u32){ 
    let mut addr = base as *mut u8;  
    for _ in 0..size { 
        let byte = getc(); 
        unsafe { 
            core::ptr::write_volatile(addr, byte);
            addr = addr.add(1); 
        }
    }
}
pub unsafe fn serial_load(base : usize )-> Result<usize,FirmwareError>{ 
    let header = read_header()?; 
    unsafe {
        load_payload(base, header.size);
        Ok(base as usize)
    } 
}