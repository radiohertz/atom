use super::{hart::{Hart}};


#[derive(Debug,PartialEq)]
pub enum Rv32iR {
    ADD,
    SUB,
    SLL,
    // SLT,
    SLTU,
    XOR,
    SRL,
    // SRA,
    OR,
    AND
}

#[derive(Debug,PartialEq)]
pub enum Rv32iI {
    ADDI,
}

/// The base RISCV ISA.
pub trait Rv32i {

    /// The `addi` instruction of type `I`.
    fn op_addi(&mut self, rd: u8, rs1: u8, imm: u16);

    /// The `add` instruction of type `R`.
    fn op_add(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `sub` instruction of type `R`.
    fn op_sub(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `ssl` instruction of type `R`.
    fn op_sll(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `slt` instruction of type `R`.
    fn op_sltu(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `xor` instruction of type `R`.
    fn op_xor(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `srl` instruction of type `R`.
    fn op_srl(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `srl` instruction of type `R`.
    fn op_sra(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `or` instruction of type `R`.
    fn op_or(&mut self, rd: u8, rs1: u8, rs2: u8);

    /// The `and` instruction of type `R`.
    fn op_and(&mut self, rd: u8, rs1: u8, rs2: u8);

 
}


impl Rv32i for Hart {

    fn op_addi(&mut self, rd: u8, rs1: u8, imm: u16) {
        let r_val = self.read_reg(rs1);
        self.write_reg(rd, r_val + imm as u64);
    }

    fn op_add(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 + v2);
    }

    fn op_sub(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 - v2);
    }

    fn op_sll(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2) & 0x1f;
        self.write_reg(rd, v1 << v2);
    }

    fn op_sltu(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        let v = if v1 < v2 {
            1
        } else {
            0
        };

        self.write_reg(rd, v);
    }

    fn op_xor(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 ^ v2);
    }

    fn op_srl(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2) | 0x1f;
        self.write_reg(rd, v1 >> v2);
    }

    fn op_or(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 | v2);
    }

    fn op_sra(&mut self, rd: u8, rs1: u8, rs2: u8) {

    }

    fn op_and(&mut self, rd: u8, rs1: u8, rs2: u8) {
        let v1 = self.read_reg(rs1);
        let v2 = self.read_reg(rs2);
        self.write_reg(rd, v1 & v2);
    }

}

