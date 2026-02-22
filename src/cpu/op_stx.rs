use crate::cpu::{
    alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY},
    bits::Word,
};
use log::debug;

impl Cpu {
    pub fn op_stx(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let value = &Word { data: self.reg_x };
        let oldpc = self.pc;

        match opcode {
            0x86 => self.store(AddressMode::ZeroPage, value, sixteen_bits_mode),
            0x96 => self.store(AddressMode::ZeroPageY, value, sixteen_bits_mode),
            0x8E => self.store(AddressMode::Absolute, value, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };
        debug!("[0x{:X}:0x{:X}] STX : X=0x{:X}", oldpc, opcode, value.data);
    }
}
