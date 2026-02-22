use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_and(&mut self, opcode: u8) {
        let value = match opcode {
            0x29 => self.fetch(AddressMode::Immediate, true),
            0x25 => self.fetch(AddressMode::ZeroPage, true),
            0x35 => self.fetch(AddressMode::ZeroPageX, true),
            0x2D => self.fetch(AddressMode::Absolute, true),
            0x3D => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0x39 => self.fetch(AddressMode::AbsoluteIndexedY, true),
            0x21 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX, true),
            0x31 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0x2F => self.fetch(AddressMode::AbsoluteLong, true),
            0x3F => self.fetch(AddressMode::AbsoluteLongIndexedX, true),
            0x32 => self.fetch(AddressMode::DirectIndirect, true),
            0x27 => self.fetch(AddressMode::DirectIndirectLong, true),
            0x37 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, true),
            0x23 => self.fetch(AddressMode::StackRelative, true),
            0x33 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.reg_a.data = self.reg_a.data & value;
        self.flag_nz(self.reg_a.data);

        debug!(
            "[0x{:X}] AND : A=0x{:X} VALUE=0x{:X} FLAGS={:b}",
            opcode, self.reg_a.data, value, self.reg_p
        );
    }
}
