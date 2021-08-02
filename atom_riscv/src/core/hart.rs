pub use super::base_isa::Rv32i;

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

	fn decode(&mut self, inst: u32) {
		match inst {
			0x13 => {
				self.addi();

			},
			0x33 => {
				self.add();
			}
			_ => {

			}
		}

	}

	fn execute(&mut self) { 
	}

	pub fn run(&mut self) {
		let inst = self.fetch();
		self.pc += 4;
		self.decode(inst);

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
			println!("{} - {}", i, v);
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
	R,
	I,
	S,
	U
}


impl InstType {
	fn which(inst: u32) -> Self {
		// isolate the opcode.
		let opcode = inst & 0x3f;
		match opcode {
			0x13 => InstType::I,
			0x33 => InstType::R,
			_ => panic!("Should'nt come here")
		}
	}
}


#[derive(Debug)]
pub struct IType {
	pub(crate) opcode: u32,
	pub(crate) rd: u8,
	pub(crate) funct3: u8,
	pub(crate) rs1: u8,
	pub(crate) imm: u16
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

