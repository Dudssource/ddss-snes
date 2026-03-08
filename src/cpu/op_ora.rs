use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_ora(&mut self, opcode: u8) {
        let value = match opcode {
            0x09 => self.fetch(AddressMode::Immediate, true),
            0x05 => self.fetch(AddressMode::ZeroPage, true),
            0x15 => self.fetch(AddressMode::ZeroPageX, true),
            0x0D => self.fetch(AddressMode::Absolute, true),
            0x1D => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0x19 => self.fetch(AddressMode::AbsoluteIndexedY, true),
            0x01 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX, true),
            0x11 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0x0F => self.fetch(AddressMode::AbsoluteLong, true),
            0x1F => self.fetch(AddressMode::AbsoluteLongIndexedX, true),
            0x12 => self.fetch(AddressMode::DirectIndirect, true),
            0x07 => self.fetch(AddressMode::DirectIndirectLong, true),
            0x17 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, true),
            0x03 => self.fetch(AddressMode::StackRelative, true),
            0x13 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.reg_a.data = self.reg_a.data | value;
        self.flag_nz(self.reg_a.data);

        debug!(
            "[0x{:X}] ORA : A=0x{:X} VALUE=0x{:X} FLAGS={:b}",
            opcode, self.reg_a.data, value, self.reg_p
        );
    }
}
