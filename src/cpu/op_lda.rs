use log::debug;

use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_lda(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let oldpc = self.pc;
        self.reg_a.data = match opcode {
            0xA9 => self.fetch(AddressMode::Immediate, sixteen_bits_mode),
            0xA5 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0xB5 => self.fetch(AddressMode::ZeroPageX, sixteen_bits_mode),
            0xAD => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0xBD => self.fetch(AddressMode::AbsoluteIndexedX, sixteen_bits_mode),
            0xB9 => self.fetch(AddressMode::AbsoluteIndexedY, sixteen_bits_mode),
            0xA1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX, sixteen_bits_mode),
            0xB1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, sixteen_bits_mode),
            0xAF => self.fetch(AddressMode::AbsoluteLong, sixteen_bits_mode),
            0xBF => self.fetch(AddressMode::AbsoluteLongIndexedX, sixteen_bits_mode),
            0xB2 => self.fetch(AddressMode::DirectIndirect, sixteen_bits_mode),
            0xA7 => self.fetch(AddressMode::DirectIndirectLong, sixteen_bits_mode),
            0xB7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, sixteen_bits_mode),
            0xA3 => self.fetch(AddressMode::StackRelative, sixteen_bits_mode),
            0xB3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_a.data);
        debug!(
            "[0x{:X}:0x{:X}] LDA : A=0x{:X} FLAGS={:b}",
            oldpc, opcode, self.reg_a.data, self.reg_p
        );
    }
}
