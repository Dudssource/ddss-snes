use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_cpy(&mut self, opcode: u8) {
        let operand = match opcode {
            0xC0 => self.fetch(AddressMode::Immediate, true),
            0xC4 => self.fetch(AddressMode::ZeroPage, true),
            0xCC => self.fetch(AddressMode::Absolute, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_y - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_y >= operand);

        debug!(
            "[0x{:X}] CPY : OPERAND=0x{:X} FLAGS={:b}",
            opcode, operand, self.reg_p
        );
    }
}
