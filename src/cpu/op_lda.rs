use log::debug;

use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_lda(&mut self, opcode: u8) {
        let oldpc = self.pc;
        self.reg_a.data = match opcode {
            0xA9 => self.fetch(AddressMode::Immediate),
            0xA5 => self.fetch(AddressMode::ZeroPage),
            0xB5 => self.fetch(AddressMode::ZeroPageX),
            0xAD => self.fetch(AddressMode::Absolute),
            0xBD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xB9 => self.fetch(AddressMode::AbsoluteIndexedY),
            0xA1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0xB1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xAF => self.fetch(AddressMode::AbsoluteLong),
            0xBF => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0xB2 => self.fetch(AddressMode::DirectIndirect),
            0xA7 => self.fetch(AddressMode::DirectIndirectLong),
            0xB7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0xA3 => self.fetch(AddressMode::StackRelative),
            0xB3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_a.data);
        debug!(
            "[0x{:X}:0x{:X}] LDA : A=0x{:X} FLAGS={:b}",
            oldpc, opcode, self.reg_a.data, self.reg_p
        );
    }
}
