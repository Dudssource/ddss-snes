use crate::cpu::alu::{Cpu, S_NEGATIVE, S_RESULT_ZERO};

impl Cpu {
    pub fn op_lsr(&mut self, opcode: u8) {
        let mut value = match opcode {
            0x4A => self.reg_a.data,
            0x46 => self.fetch(crate::cpu::alu::AddressMode::ZeroPage),
            0x56 => self.fetch(crate::cpu::alu::AddressMode::ZeroPageX),
            0x4E => self.fetch(crate::cpu::alu::AddressMode::Absolute),
            0x5E => self.fetch(crate::cpu::alu::AddressMode::AbsoluteIndexedX),
            _ => panic!("invalid opcode {}", opcode),
        };

        // carry flag
        self.flag_c((value & 0x1) > 0);

        // shift right by 1
        value = value >> 1;

        // negative and zero flags
        self.flag(S_NEGATIVE, false);
        self.flag(S_RESULT_ZERO, value == 0);
    }
}
