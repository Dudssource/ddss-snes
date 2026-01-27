use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_bit(&mut self, opcode: u8) {
        let value = match opcode {
            0x24 => self.fetch(AddressMode::ZeroPage),
            0x2C => self.fetch(AddressMode::Absolute),
            0x89 => self.fetch(AddressMode::Immediate),
            0x34 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0x3C => self.fetch(AddressMode::AbsoluteIndexedX),
            _ => panic!("invalid opcode {}", opcode),
        };

        let result = self.reg_a.data & value;
        self.flag_v((self.emulation && (value & 0x40) > 0) || (value & 0x4000) > 0);
        self.flag_nz(result);
    }
}
