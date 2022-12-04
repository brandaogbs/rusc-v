use std::env;
use std::io;
use std::fs::File;
use std::io::Read;

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

/// DRAM size (128MiB)
pub const DRAM_SIZE: u64 = 1024*1024*128;

#[derive(FromPrimitive)]
enum Opcode {
    Addi = 0x13,
    Add = 0x33,
}

struct Cpu {
    regs: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0;32];
        regs[2] = DRAM_SIZE;

        Self { regs, pc: 0, dram: code, }
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.dram[index] as u32)
                | ((self.dram[index+1] as u32) << 8)
                | ((self.dram[index+2] as u32) << 16)
                | ((self.dram[index+3] as u32) << 24);
    }

    fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x7F;
        let rd = ((inst >> 7) & 0x1F) as usize;
        let rs1 = ((inst >> 15) & 0x1F) as usize;
        let rs2 = ((inst >> 20) & 0x1F) as usize;

        self.regs[0] = 0;

        match FromPrimitive::from_u32(opcode) {
            Some(Opcode::Addi) => {
                let imm = ((inst & 0xFFF0_0000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            },
            Some(Opcode::Add) => {
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            },
            _ => {
                dbg!("BadOpcode: {}", opcode);
            },
        }
    }

    fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];

        for i in (0..32).step_by(4) {
            output = format!("{}\n{}", 
                output, 
                format!("x{:02}({})={:<#10x}\tx{:02}({})={:<#10x}\tx{:02}({})={:<#10x}\tx{:02}({})={:<#10x}",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            )
        }

        println!("{}", output);
    }
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: riscv <filename>");
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();

    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.dram.len() as u64 {
        // 1. Fetch
        let inst = cpu.fetch();

        cpu.pc = cpu.pc + 4;
        // 2. Decode
        
        // 3. Execute  
        cpu.execute(inst);
    }
    cpu.dump_registers();
    
    Ok(())
}

