use crate::cpu::alu::{AddressMode, Cpu};
use log::debug;

impl Cpu {
    pub fn op_ldx(&mut self, opcode: u8) {
        debug!("LDX #${:X} : PC 0x{:X}", opcode, self.pc);
        self.reg_x = match opcode {
            0xA2 => self.fetch(AddressMode::Immediate),
            0xA6 => self.fetch(AddressMode::ZeroPage),
            0xB6 => self.fetch(AddressMode::ZeroPageY),
            0xAE => self.fetch(AddressMode::Absolute),
            0xBE => self.fetch(AddressMode::AbsoluteIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };


        self.flag_nz(self.reg_x);
    }
}
