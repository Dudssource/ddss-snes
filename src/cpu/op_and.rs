use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_and(&mut self, opcode: u8) {
        let value = match opcode {
            0x29 => self.fetch(AddressMode::Immediate),
            0x25 => self.fetch(AddressMode::ZeroPage),
            0x35 => self.fetch(AddressMode::ZeroPageX),
            0x2D => self.fetch(AddressMode::Absolute),
            0x3D => self.fetch(AddressMode::AbsoluteIndexedX),
            0x39 => self.fetch(AddressMode::AbsoluteIndexedY),
            0x21 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0x31 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0x2F => self.fetch(AddressMode::AbsoluteLong),
            0x3F => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0x32 => self.fetch(AddressMode::DirectIndirect),
            0x27 => self.fetch(AddressMode::DirectIndirectLong),
            0x37 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0x23 => self.fetch(AddressMode::StackRelative),
            0x33 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
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
