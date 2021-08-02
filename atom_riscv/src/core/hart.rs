pub use super::base_isa::{Rv32i,Rv32iI,Rv32iR};

/// A RISC-V Hardware thread.
/// Contains its own registers and program counter.
pub struct Hart {
    /// 32 General purpose registers.
    x: [u64;32],
    /// 64 bit program counter.
    pc: u64,
    /// The application code that hart is going to execute.
    code: Option<Vec<u8>>
}

macro_rules! disassemble {
    ($block: block) => (
        if cfg!(feature = "disasm") {
            $block;
            // return;
        }
    )
}


impl Hart {
    /// Create a new hart (hardware thread).
    /// `code` - the code the thread is going to execute.
    pub fn new(code: Option<Vec<u8>>) -> Self {
        Hart{
            x: [0;32],
            pc: 0x0,
            code
        }
    }

    /// Fetch the next instruction.
    fn fetch(&self) -> u32 {
        let idx = self.pc as usize;
        assert!(idx % 4 == 0, "Unaligned instruction memory access");

        let code = self.code.as_ref().unwrap();

        (code[idx] as u32) |
        (code[idx + 1] as u32) << 8 |
        (code[idx + 2] as u32) << 16 |
        (code[idx + 3] as u32) << 24
    }

    fn decode(&mut self, inst: u32) -> InstType {
        InstType::which(inst)
    }

    fn execute(&mut self, inst: InstType) { 
        match inst {
            InstType::R(opcode,rd,funct3,rs1,rs2,funct7) => {
                disassemble!({
                    println!("{:?} x{}, x{}, x{}", opcode, rd, rs1, rs2);
                });
                match opcode {
                    Rv32iR::ADD => self.op_add(rd,rs1,rs2),
                    Rv32iR::SUB => self.op_sub(rd,rs1,rs2),
                    Rv32iR::SLL => self.op_sll(rd,rs1,rs2),
                    Rv32iR::XOR => self.op_xor(rd,rs1,rs2),
                    Rv32iR::SRL => self.op_srl(rd,rs1,rs2),
                    Rv32iR::SLTU => self.op_sltu(rd,rs1,rs2),
                    Rv32iR::AND => self.op_and(rd,rs1,rs2),
                    Rv32iR::OR => self.op_or(rd,rs1,rs2),
                } 
            },
            InstType::I(opcode,rd,funct3,rs1,imm) => {
                disassemble!({
                    println!("{:?} x{}, x{}, {}", opcode, rd, rs1, imm);
                });

                match opcode {
                    Rv32iI::ADDI => self.op_addi(rd,rs1,imm)
                }
            },
            _ => panic!("Not yet")
        }
    }

    pub fn run(&mut self) {
        let len = self.code.as_ref().unwrap().len();

        loop {
            if self.pc >= len as u64 {
                break;
            }
            let encoded_inst = self.fetch();
            self.pc += 4;
            let decoded_inst = self.decode(encoded_inst);
            self.execute(decoded_inst);
        }

    }

    /// Read and write to the registers using the `RWreg`
    #[inline]
    fn reg_rw(&mut self, op: RWreg) -> Option<u64> {
        match op {
            RWreg::Read(i) => Some(*self.x.get(i as usize).unwrap()),
            RWreg::Write(0,_) => panic!("Dont do that! cant write to x0"),
            RWreg::Write(reg,val) => {
                *self.x.get_mut(reg as usize).unwrap() = val;
                None
            }
        }
    }


    #[inline]
    pub fn read_reg(&mut self, reg: u8) -> u64 {
        self.reg_rw(RWreg::Read(reg)).unwrap()
    }

    #[inline]
    pub fn write_reg(&mut self, reg: u8, val: u64) {
        assert_eq!(self.reg_rw(RWreg::Write(reg, val)), None);
    }

    pub fn debug(&self) {
        for (i,v) in self.x.iter().enumerate() {
            println!("x{} - {}", i, v);
        }
    }
}

pub enum RWreg {
    Read(u8),
    Write(u8,u64)
}


/// Various instruction encoding types used by RISC-V.
#[derive(Debug,PartialEq)]
pub enum InstType {
    /// The `R` instruction type.
    /// Denoted as `R(opcode,rd,funct3,rs1,rs2,funct7)`
    R(Rv32iR,u8,u8,u8,u8,u8),
    /// The `I` instruction type.
    /// Denoted as `I(opcode,rd,funct3,rs1,imm)`
    I(Rv32iI,u8,u8,u8,u16),
    S,
    U,
    UNK
}


impl InstType {
    fn which(inst: u32) -> Self {
        // isolate the opcode.
        let opcode = (inst & 0x7f) as u8;
        let rd_imm = (inst >> 7 & 0x1f) as u8;
        let funct3 = (inst >> 12 & 0x7) as u8;
        let rs1_imm = (inst >> 15 & 0x1f) as u8;

        match opcode {
            0x13 => {
                let imm = (inst >> 20) as u16;
                let instr = match funct3 {
                    0x0 => Rv32iI::ADDI,
                    _ => panic!("Unimpl imm instr")
                };
                InstType::I(instr,rd_imm,funct3,rs1_imm,imm)
            },
            0x33 => {
                let funct7 = (inst >> 25) as u8;
                let rs2 = (inst >> 20 & 0x1f) as u8;
                let instr = match (funct3,funct7) {
                    (0,0) => Rv32iR::ADD,
                    (0x0,0x20) => Rv32iR::SUB,
                    (0x1,0) => Rv32iR::SLL,
                    (0x4,0) => Rv32iR::XOR,
                    (0x5,0) => Rv32iR::SRL,
                    (0x3,0) => Rv32iR::SLTU,
                    (0x6,0) => Rv32iR::OR,
                    (0x7,0) => Rv32iR::AND,
                    (_,_) => {
                        // println!("Eh what, {} {} {} {} {} {}", opcode, rd_imm,funt3,rs1_imm,rs2,funt7);
                        panic!("Unimpl R instr");
                    }
                };
                InstType::R(instr,rd_imm,funct3,rs1_imm,rs2,funct7)
            },
            _ => panic!("Should'nt come here")
        }
    }
}


#[test]
#[should_panic]
fn test_register_write_to_none() {
    let mut hart = Hart::new(None);
    hart.reg_rw(RWreg::Write(32,0x1337));
}

#[test]
#[should_panic]
fn test_register_write_to_x9() {
    let mut hart = Hart::new(None);
    hart.reg_rw(RWreg::Write(0,0x1));
}
