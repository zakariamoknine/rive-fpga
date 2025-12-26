use crate::mmio::*;
use crate::uart8250::getc;
use crate::error::FirmwareError;
const SERIAL_MAGIC: u32 = 0x52444C52; 
const MAX_SIZE: u32 = ddr2::SIZE;

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
        })   
    }
    pub fn validate(&self)-> Result<(), FirmwareError> { 
        if self.magic != SERIAL_MAGIC {
            return Err(FirmwareError::UartInvalidMagicNumber); 
        }
        if self.size ==0 || self.size > MAX_SIZE { 
            return Err(FirmwareError::UartSizeOverflow);
        }
        Ok(())
    }

} 
pub unsafe fn read_header() -> Result<SerialHeader, FirmwareError> { 
        const HEADER_SIZE:usize = core::mem::size_of::<SerialHeader>(); 
        let mut buf = [u8; HEADER_SIZE]; 
        for i in 0..HEADER_SIZE { 
            unsafe {
                buf[i]= getc();   
            } 
        }
        let header = SerialHeader::parse(&buf); 
        header.validate()?; 
        Ok(header)
    } 
pub unsafe fn load_payload(base :usize ,size: u32){ 
    let mut addr = base as *mut u8;  
    for _ in 0..size {  
        unsafe { 
            let byte = getc();
            iowrite8(addr, byte);
            addr = addr.add(1); 
        }
    }
}
pub unsafe fn load(base : usize )-> Result<usize,FirmwareError>{ 
    unsafe { 
        let header = read_header()?; 
        load_payload(base, header.size);
        Ok(base as usize)
    }  
    
 
}