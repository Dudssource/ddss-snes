use log::debug;

use crate::cpu::{
    alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY},
    bits::Word,
};

impl Cpu {
    pub fn op_stz(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let zero_value = &Word { data: 0x0 };
        let oldpc = self.pc;
        match opcode {
            0x9C => self.store(AddressMode::Absolute, zero_value, sixteen_bits_mode),
            0x9E => self.store(AddressMode::AbsoluteIndexedX, zero_value, sixteen_bits_mode),
            0x64 => self.store(AddressMode::ZeroPage, zero_value, sixteen_bits_mode),
            0x74 => self.store(AddressMode::ZeroPageX, zero_value, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        }
        debug!("[0x{:X}:0x{:X}] STZ", oldpc, opcode);
    }
}
