use crate::cpu::alu::{Cpu, AddressMode};

impl Cpu {
    pub fn op_cmp(&mut self, opcode: u8) {
        let operand = match opcode {
            0xC9 => self.fetch(AddressMode::Immediate),
            0xC5 => self.fetch(AddressMode::ZeroPage),
            0xD5 => self.fetch(AddressMode::ZeroPageX),
            0xCD => self.fetch(AddressMode::Absolute),
            0xDD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xD9 => self.fetch(AddressMode::AbsoluteIndexedY),
            0xC1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xD1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xCF => self.fetch(AddressMode::AbsoluteLong),
            0xDF => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0xD2 => self.fetch(AddressMode::DirectIndirect),
            0xC7 => self.fetch(AddressMode::DirectIndirectLong),
            0xD7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0xC3 => self.fetch(AddressMode::StackRelative),
            0xD3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_a.data - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_a.data >= operand);
    }
}