use crate::cpu::alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY};
use log::debug;

impl Cpu {
    pub fn op_sta(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let value = &self.reg_a.clone();
        let oldpc = self.pc;

        match opcode {
            0x85 => self.store(AddressMode::ZeroPage, value, sixteen_bits_mode),
            0x95 => self.store(AddressMode::ZeroPageX, value, sixteen_bits_mode),
            0x8D => self.store(AddressMode::Absolute, value, sixteen_bits_mode),
            0x9D => self.store(AddressMode::AbsoluteIndexedX, value, sixteen_bits_mode),
            0x99 => self.store(AddressMode::AbsoluteIndexedY, value, sixteen_bits_mode),
            0x81 => self.store(
                AddressMode::ZeroPageDirectIndexedIndirectX,
                value,
                sixteen_bits_mode,
            ),
            0x91 => self.store(
                AddressMode::ZeroPageDirectIndirectIndexedY,
                value,
                sixteen_bits_mode,
            ),
            0x8F => self.store(AddressMode::AbsoluteLong, value, sixteen_bits_mode),
            0x9F => self.store(AddressMode::AbsoluteLongIndexedX, value, sixteen_bits_mode),
            0x92 => self.store(AddressMode::DirectIndirect, value, sixteen_bits_mode),
            0x87 => self.store(AddressMode::DirectIndirectLong, value, sixteen_bits_mode),
            0x97 => self.store(
                AddressMode::ZeroPageDirectIndirectIndexedLong,
                value,
                sixteen_bits_mode,
            ),
            0x83 => self.store(AddressMode::StackRelative, value, sixteen_bits_mode),
            0x93 => self.store(
                AddressMode::StackRelativeIndirectIndexedY,
                value,
                sixteen_bits_mode,
            ),
            _ => panic!("invalid opcode {}", opcode),
        };
        debug!("[0x{:X}:0x{:X}] STA : A=0x{:X}", oldpc, opcode, value.data);
    }
}
