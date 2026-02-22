use crate::cpu::alu::{AddressMode, Cpu, S_INDEX_REGISTERS};
use log::debug;

impl Cpu {
    pub fn op_ldx(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_INDEX_REGISTERS) == 0;
        self.reg_x = match opcode {
            0xA2 => self.fetch(AddressMode::Immediate, sixteen_bits_mode),
            0xA6 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0xB6 => self.fetch(AddressMode::ZeroPageY, sixteen_bits_mode),
            0xAE => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0xBE => self.fetch(AddressMode::AbsoluteIndexedY, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_x);
        debug!(
            "[0x{:X}:0x{:X}] LDX : X=0x{:X} FLAGS={:b}",
            self.pc, opcode, self.reg_x, self.reg_p
        );
    }
}
