use crate::dram::*;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
        }
    }

    pub fn load(&self, addr: u32, size: u32) -> Result<u32, ()> {
        if DRAM_BASE <= addr {
            return self.dram.load(addr, size);
        }
        Err(())
    }

    pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()> {
        if DRAM_BASE <= addr {
            return self.dram.store(addr, size, value);
        }
        Err(())
    }
}