use crate::bus::*;
use crate::dram::*;

use std::convert::From;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0;32];
        regs[2] = DRAM_BASE + DRAM_SIZE;

        Self { regs, pc: DRAM_BASE, bus: Bus::new(code), }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, ()> {
        self.bus.load(addr, size)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        self.bus.store(addr, size, value)
    }

    pub fn fetch(&mut self) -> Result<u64, ()> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(_e) => Err(()),
        }
    }

    pub fn execute(&mut self, inst: u64) -> Result<(), ()> {
        fn sing_extend(x:i32, l:i32) -> i32 {
            if x >> (l-1) == 1 {
                return -((1 << l)-x);
            }
            return x;
        }

        let b = |s:u32, e:u32| (inst >> e) & ((1<<(s-e+1))-1);

        // 2. Decode
        let opcode = b(6, 0);
        let rd = b(11, 7) as usize;
        let rs1 = b(19, 15) as usize;
        let rs2 = b(24, 20) as usize;
        let funct3 = b(14, 12);
        let funct7 = b(31, 25);

        // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
        let imm_j = sing_extend((
            (b(32, 31) << 20)
            | (b(30, 21) << 1)
            | (b(20, 20) << 11)
            | (b(19,12) << 12)) as i32, 21) as u32 as u64;
        
        // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
        let imm_b = sing_extend((
            (b(32, 31) << 12)
            | (b(30, 25) << 5)
            | (b(11, 8) << 1)
            | (b(7,7) << 11)) as i32, 13) as u64;

        // imm[11:0] = inst[31:20]
        let imm_i = sing_extend(b(31, 20) as i32, 12) as u32 as u64;

        // imm[20:0] = inst[31:12]
        let imm_u =  b(31, 12) as u32 as u64;

        println!("pc:{:#x} opcode:{:#x} funct3:{:#x} funct7:{:#x} ", self.pc.wrapping_sub(4), opcode, funct3, funct7);
        println!("reg[rd]:{:#x} reg[rs1]:{:#x} reg[rs2]:{:#x} ", self.regs[rd], self.regs[rs1], self.regs[rs2]);
        println!("imm_i:{:#x} imm_b:{:#x} imm_u:{:#x} imm_j:{:#x} ", imm_i, imm_b, imm_u, imm_j);
        self.regs[0] = 0;

        // http://pages.hmc.edu/harris/ddca/ddcarv/DDCArv_AppB_Harris.pdf
        match opcode {
            0x13 => { // I-type
                match funct3 {
                    0x0 => { // ADDI
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm_i) as u32 as u64;
                    },
                    0x1 => { // SLLI
                        let shamt = self.regs[rs2];
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm_i<<shamt);
                    },
                    0x6 => { // ORI
                         self.regs[rd] = self.regs[rs1] | imm_i;
                    },
                    0x7 => { // ANDI
                        self.regs[rd] = (self.regs[rs1] & imm_i) as u32 as u64;
                    },
                    _ => {
                        println!("not implemented yet: opcode {:#x} funct3 {:#x}", opcode, funct3);
                        return Err(());                    
                    }
                }
                return Ok(());
            },
            0x17 => { // AUIPC
                self.regs[rd] = self.pc.wrapping_add(imm_j).wrapping_sub(4);
                return Ok(())
            },
            0x1b => { // I
                match funct3 {
                    0x0 => {
                        let v = sing_extend((imm_i << 12) as i32, 12) as u64;
                        self.regs[rd] = self.regs[rs1].wrapping_add(v) as i32 as i64 as u64;
                    },
                    _ => {
                        println!("not implemented yet: opcode {:#x} funct3 {:#x}", opcode, funct3);
                        return Err(());                    
                    }
                }
                return Ok(())
            },
            0x33 => { // R-type
                match (funct3, funct7) {
                    (0x0, 0x00) => { // ADD
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]) as u32 as u64;
                    },
                    (0x0, 0x20) => { // SUB
                        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]) as u32 as u64;
                    },
                    (0x7, 0x00) => { // AND
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2] as u32 as u64;
                    },
                    _ => {
                        println!("not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}", opcode, funct3, funct7);
                        return Err(());  
                    }
                }
                return Ok(())
            },
            0x37 => { // LUI
                self.regs[rd] = imm_u << 12;
                return Ok(())
            },
            0x6f => { // JAL
                self.regs[rd] = self.pc.wrapping_add(4);
                self.pc = self.pc.wrapping_add(imm_j).wrapping_sub(4);
                return Ok(())
            },
            0x73 => { // SYSTEM
                return Ok(())
            },
            0x63 => { // BRANCH
                match funct3 {
                    0x0 => { //BEQ
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm_b).wrapping_sub(4);
                        }
                    },
                    0x1 => { // BNE
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm_b).wrapping_sub(4);
                        }
                    },
                    0x4 => { // BLE
                        if (self.regs[rs1] as i32 as i64) < (self.regs[rs2] as i32 as i64) {
                            self.pc = self.pc.wrapping_add(imm_b).wrapping_sub(4);
                        }
                    },
                    0x5 => { // BGE
                        if (self.regs[rs1] as i32 as i64) >= (self.regs[rs2] as i32 as i64) {
                            self.pc = self.pc.wrapping_add(imm_b).wrapping_sub(4);
                        }
                    }
                    0x7 => { // BGEU
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm_b).wrapping_sub(4);
                        }
                    }
                    _ => {
                        println!("not implemented yet: opcode {:#x} funct3 {:#x}", opcode, funct3);
                        return Err(());                    
                    }
                }
                return Ok(())
            },
            0xf => { // FENCE
                return Ok(())
            },
            _ => {
                println!("not implemented yet: opcode {:#x} funct3 {:#x}", opcode, funct3);
                return Err(());
            },
        }
    }

    pub fn dump_registers(&self) {
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
