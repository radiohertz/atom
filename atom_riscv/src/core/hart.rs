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


impl Hart {
    /// Create a new hart (hardware thread).
    /// `code` - the code the thread is going to execute.
    pub fn new(code: Option<Vec<u8>>) -> Self {
        Hart{
            x: [0;32],
            pc: 0x0,
            code: code
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
        let opcode = inst & 0x3f;
        let decoded = InstType::which(inst);
        decoded
    }

    fn execute(&mut self, inst: InstType) { 
        match inst {
            InstType::R(opcode,rd,funct3,rs1,rs2,funct7) => {
               match opcode {
                Rv32iR::ADD => self.op_add(rd,rs1,rs2)
               } 
            },
            InstType::I(opcode,rd,funct3,rs1,imm) => {
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
    fn reg_rw(&mut self, op: RWreg) -> Option<u64> {
        match op {
            RWreg::Read(i) => Some(*self.x.get(i as usize).unwrap()),
            RWreg::Write(reg,val) => {
                *self.x.get_mut(reg as usize).unwrap() = val;
                None
            }
        }
    }


    pub fn read_reg(&mut self, reg: u8) -> u64 {
        self.reg_rw(RWreg::Read(reg)).unwrap()
    }

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
        let funt3 = (inst >> 12 & 0x3) as u8;
        let rs1_imm = (inst >> 15 & 0x1f) as u8;

        match opcode {
            0x13 => {
                let imm = (inst >> 20) as u16;
                match funt3 {
                    0x0 => {
                       InstType::I(Rv32iI::ADDI,rd_imm,funt3,rs1_imm,imm)
                    }
                    _ => InstType::UNK
                }
            },
            0x33 => {
                let funt7 = (inst >> 25) as u8;
                let rs2 = (inst >> 20 & 0x1f) as u8;
                match ((funt3,funt7)) {
                    ((0,0)) => {
                        // println!("ADD");
                        InstType::R(Rv32iR::ADD,rd_imm,funt3,rs1_imm,rs2,funt7)
                    },
                    ((_,_)) => {
                        InstType::UNK
                    }
                }
            },
            _ => panic!("Should'nt come here")
        }
    }
}


#[test]
fn test_correct_decoded_instruction_type() {
    assert_eq!(InstType::which(0x13), InstType::I);
    assert_eq!(InstType::which(0x33), InstType::R);
}

#[test]
#[should_panic]
fn test_register_write_to_none() {
    let mut hart = Hart::new(None);
    hart.reg_rw(RWreg::Write(32,0x1337));
}

