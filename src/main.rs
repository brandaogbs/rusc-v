mod bus;
mod cpu;
mod dram;

use dram::DRAM_SIZE;
use std::fs::File;
use std::io::{Read, Result};

use crate::cpu::*;
use crate::dram::DRAM_BASE;

fn main() -> Result<()> {
    let mut code = vec![0; DRAM_SIZE as usize];
    // TODO: loop over the programs
    let file_name = "riscv-tests/isa/rv32ui-p-sw";
    let mut f = File::open(file_name).expect("Failed to open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to read file");

    let file = goblin::elf::Elf::parse(&buffer).expect("Failed to parse ELF file");

    for ph in file.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let data = &buffer[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize];

            let start = ph.p_paddr-DRAM_BASE as u64;
            let stop= start+ph.p_filesz;
            code[start as usize..stop as usize].copy_from_slice(&data);
        }
    }    
    
    // println!("{:x?}", code);
    let mut cpu = Cpu::new(code);

    loop {
        // 1. Fetch
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(_) => break,
        };

        cpu.pc = cpu.pc.wrapping_add(4);

        // 2. Decode
        // 3. Execute
        match cpu.execute(inst) {
            Ok(_) => {},
            Err(_) => break,
        }
        cpu.dump_registers();
        println!("----");
        if cpu.pc == 0 {
            break;
        }
    }
    cpu.dump_registers();
    
    Ok(())
}

