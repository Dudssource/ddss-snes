use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_rep(&mut self) {
        let mut mask = (self.fetch(AddressMode::Immediate) & 0xFF) as u8;
        if self.emulation {
            mask &= 0xCF;
        }
        self.reg_p = self.reg_p & (!mask);
    }
}