mod bus;
mod cpu;
mod dram;

use dram::DRAM_SIZE;
use std::fs::File;
use std::io::{Read, Result};

use crate::cpu::*;

fn main() -> Result<()> {
    let mut code = Vec::new();
    // TODO: loop over the programs
    let file_name = "../../riscv/riscv-tests/isa/rv32ui-p-bgeu";
    let mut f = File::open(file_name).expect("Failed to open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to read file");

    let file = goblin::elf::Elf::parse(&buffer).expect("Failed to parse ELF file");

    for ph in file.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            code.extend(&buffer[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize].to_vec());
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

