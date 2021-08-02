use super::{hart::{Hart, RWreg}};


pub enum Rv32iInst {
    ADDI(u8, u8,u16),
    ADD(u8,u8,u8)
}


/// The base RISCV ISA.
pub trait Rv32i {

    /// The `addi` instruction of type `I`.
    fn addi(&mut self, rd: u8, rs1: u8, imm: u16);

    /// The `add` instruction of type `R`.
    fn add(&mut self, rd: u8, rs1: u8, rs2: u8);
}


impl Rv32i for Hart {

    fn addi(&mut self, rd: u8, rs1: u8, imm: u16) {
        let r_val = self.read_reg(rs1);
        self.write_reg(rd, r_val + imm as u64);
    }

    /// The `add` instruction of type `R`.
    fn add(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 + v2);
    }
}

