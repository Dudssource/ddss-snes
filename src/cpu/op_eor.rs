use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_eor(&mut self, opcode: u8) {
        let value = match opcode {
            0x49 => self.fetch(AddressMode::Immediate, true),
            0x45 => self.fetch(AddressMode::ZeroPage, true),
            0x55 => self.fetch(AddressMode::ZeroPageX, true),
            0x4D => self.fetch(AddressMode::Absolute, true),
            0x5D => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0x59 => self.fetch(AddressMode::AbsoluteIndexedY, true),
            0x41 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX, true),
            0x51 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0x4F => self.fetch(AddressMode::AbsoluteLong, true),
            0x5F => self.fetch(AddressMode::AbsoluteLongIndexedX, true),
            0x52 => self.fetch(AddressMode::DirectIndirect, true),
            0x47 => self.fetch(AddressMode::DirectIndirectLong, true),
            0x57 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, true),
            0x43 => self.fetch(AddressMode::StackRelative, true),
            0x53 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.reg_a.data = self.reg_a.data ^ value;
        self.flag_nz(self.reg_a.data);

        debug!(
            "[0x{:X}] EOR : A=0x{:X} FLAGS={:b}",
            opcode, self.reg_a.data, self.reg_p
        );
    }
}
