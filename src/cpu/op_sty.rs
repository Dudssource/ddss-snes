use crate::cpu::{
    alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY},
    bits::Word,
};
use log::debug;

impl Cpu {
    pub fn op_sty(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let value = &Word { data: self.reg_y };
        let oldpc = self.pc;

        match opcode {
            0x84 => self.store(AddressMode::ZeroPage, value, sixteen_bits_mode),
            0x94 => self.store(AddressMode::ZeroPageX, value, sixteen_bits_mode),
            0x8C => self.store(AddressMode::Absolute, value, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };
        debug!("[0x{:X}:0x{:X}] STY : Y=0x{:X}", oldpc, opcode, value.data);
    }
}
