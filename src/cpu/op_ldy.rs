use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_ldy(&mut self, opcode: u8) {
        self.reg_y = match opcode {
            0xA0 => self.fetch(AddressMode::Immediate),
            0xA4 => self.fetch(AddressMode::ZeroPage),
            0xB4 => self.fetch(AddressMode::ZeroPageX),
            0xAC => self.fetch(AddressMode::Absolute),
            0xBC => self.fetch(AddressMode::AbsoluteIndexedX),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_y);
        debug!(
            "[0x{:X}] LDY : Y=0x{:X} FLAGS={:b}",
            opcode, self.reg_y, self.reg_p
        );
    }
}
