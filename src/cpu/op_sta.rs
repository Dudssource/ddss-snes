use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_sta(&mut self, opcode: u8) {
        let value = &self.reg_a.clone();
        match opcode {
            0x85 => self.store(AddressMode::ZeroPage, value),
            0x95 => self.store(AddressMode::ZeroPageX, value),
            0x8D => self.store(AddressMode::Absolute, value),
            0x9D => self.store(AddressMode::AbsoluteIndexedX, value),
            0x99 => self.store(AddressMode::AbsoluteIndexedY, value),
            0x81 => self.store(AddressMode::ZeroPageDirectIndexedIndirectX, value),
            0x91 => self.store(AddressMode::ZeroPageDirectIndirectIndexedY, value),
            0x8F => self.store(AddressMode::AbsoluteLong, value),
            0x9F => self.store(AddressMode::AbsoluteLongIndexedX, value),
            0x92 => self.store(AddressMode::DirectIndirect, value),
            0x87 => self.store(AddressMode::DirectIndirectLong, value),
            0x97 => self.store(AddressMode::ZeroPageDirectIndirectIndexedLong, value),
            0x83 => self.store(AddressMode::StackRelative, value),
            0x93 => self.store(AddressMode::StackRelativeIndirectIndexedY, value),
            _ => panic!("invalid opcode {}", opcode),
        };
    }
}