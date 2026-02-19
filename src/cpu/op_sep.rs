use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;
use log::debug;

impl Cpu {
    pub fn op_sep(&mut self) {
        let mut mask = (self.fetch(AddressMode::Immediate) & 0xFF) as u8;
        debug!("SEP #${:X} : PC 0x{:X}", mask, self.pc);
        if self.emulation {
            mask &= 0xCF;
        }
        self.reg_p = self.reg_p | mask;
    }
}
