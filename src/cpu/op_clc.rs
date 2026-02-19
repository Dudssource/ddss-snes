use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_clc(&mut self) {
        debug!("CLC : PC 0x{:X}", self.pc);
        self.flag_c(false);
    }
}
