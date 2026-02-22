use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_cmp(&mut self, opcode: u8) {
        let operand = match opcode {
            0xC9 => self.fetch(AddressMode::Immediate, true),
            0xC5 => self.fetch(AddressMode::ZeroPage, true),
            0xD5 => self.fetch(AddressMode::ZeroPageX, true),
            0xCD => self.fetch(AddressMode::Absolute, true),
            0xDD => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0xD9 => self.fetch(AddressMode::AbsoluteIndexedY, true),
            0xC1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0xD1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0xCF => self.fetch(AddressMode::AbsoluteLong, true),
            0xDF => self.fetch(AddressMode::AbsoluteLongIndexedX, true),
            0xD2 => self.fetch(AddressMode::DirectIndirect, true),
            0xC7 => self.fetch(AddressMode::DirectIndirectLong, true),
            0xD7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, true),
            0xC3 => self.fetch(AddressMode::StackRelative, true),
            0xD3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_a.data - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_a.data >= operand);

        debug!(
            "[0x{:X}] CMP : OPERAND=0x{:X} FLAGS={:b}",
            opcode, operand, self.reg_p
        );
    }
}
