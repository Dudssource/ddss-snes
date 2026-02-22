use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_asl(&mut self, opcode: u8) {
        let mut value = match opcode {
            0x0A => self.reg_a.data,
            0x06 => self.fetch(crate::cpu::alu::AddressMode::ZeroPage, true),
            0x16 => self.fetch(crate::cpu::alu::AddressMode::ZeroPageX, true),
            0x0E => self.fetch(crate::cpu::alu::AddressMode::Absolute, true),
            0x1E => self.fetch(crate::cpu::alu::AddressMode::AbsoluteIndexedX, true),
            _ => panic!("invalid opcode {}", opcode),
        };

        // carry flag
        self.flag_c((self.emulation && (value & 0x80) > 0) || (value & 0x8000) > 0);

        // shift left by 1
        value = value << 1;

        // negative and zero flags
        self.flag_nz(value);

        debug!(
            "[0x{:X}] ASL : VALUE=0x{:X} FLAGS={:b}",
            opcode, value, self.reg_p
        );
    }
}
