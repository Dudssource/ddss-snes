use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_cpx(&mut self, opcode: u8) {
        let operand = match opcode {
            0xE0 => self.fetch(AddressMode::Immediate, true),
            0xE4 => self.fetch(AddressMode::ZeroPage, true),
            0xEC => self.fetch(AddressMode::Absolute, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_x - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_x >= operand);

        debug!(
            "[0x{:X}] CPX : OPERAND=0x{:X} FLAGS={:b}",
            opcode, operand, self.reg_p
        );
    }
}
