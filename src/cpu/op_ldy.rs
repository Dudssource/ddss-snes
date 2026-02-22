use log::debug;

use crate::cpu::alu::{AddressMode, Cpu, S_INDEX_REGISTERS};

impl Cpu {
    pub fn op_ldy(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_INDEX_REGISTERS) == 0;
        let oldpc = self.pc;
        self.reg_y = match opcode {
            0xA0 => self.fetch(AddressMode::Immediate, sixteen_bits_mode),
            0xA4 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0xB4 => self.fetch(AddressMode::ZeroPageX, sixteen_bits_mode),
            0xAC => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0xBC => self.fetch(AddressMode::AbsoluteIndexedX, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_y);
        debug!(
            "[0x{:X}:0x{:X}] LDY : Y=0x{:X} FLAGS={:b}",
            oldpc, opcode, self.reg_y, self.reg_p
        );
    }
}
