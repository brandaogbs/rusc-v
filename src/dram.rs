use crate::bus::*;

// pub const DRAM_SIZE: u64 = 1024*1024*128;
pub const DRAM_SIZE: u64 = 0x4000;
pub const DRAM_BASE: u64 = 0x8000_0000;

#[derive(Debug)]
pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Dram {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(()),
        }
    }

    fn load8(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return self.dram[index] as u64;
    }

    fn load16(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u64) | ((self.dram[index+1] as u64) << 8);
    }

    fn load32(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        let tmp = (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24);
        // println!("inst: {:#x}", tmp);
        return tmp;
    }
    
    fn load64(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u64) 
                | ((self.dram[index+1] as u64) << 8)
                | ((self.dram[index+2] as u64) << 16)
                | ((self.dram[index+3] as u64) << 24)
                | ((self.dram[index+4] as u64) << 32)
                | ((self.dram[index+5] as u64) << 40)
                | ((self.dram[index+6] as u64) << 48)
                | ((self.dram[index+7] as u64) << 56)
    }

    fn store8(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xFF) as u8;
    }
    
    fn store16(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xFF) as u8;
        self.dram[index+1] = ((value >> 8) & 0xFF) as u8;
    }

    fn store32(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xFF) as u8;
        self.dram[index+1] = ((value >> 8) & 0xFF) as u8;
        self.dram[index+2] = ((value >> 16) & 0xFF) as u8;
        self.dram[index+3] = ((value >> 24) & 0xFF) as u8;
    }

    fn store64(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xFF) as u8;
        self.dram[index+1] = ((value >> 8) & 0xFF) as u8;
        self.dram[index+2] = ((value >> 16) & 0xFF) as u8;
        self.dram[index+3] = ((value >> 24) & 0xFF) as u8;
        self.dram[index+4] = ((value >> 32) & 0xFF) as u8;
        self.dram[index+5] = ((value >> 40) & 0xFF) as u8;
        self.dram[index+6] = ((value >> 48) & 0xFF) as u8;
        self.dram[index+7] = ((value >> 56) & 0xFF) as u8;
    }
}
