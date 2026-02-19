use crate::cpu::alu::{Cpu, S_IRQ_DISABLE};
use log::debug;

impl Cpu {
    pub fn op_sei(&mut self) {
        debug!("SEI : PC 0x{:X}", self.pc);
        self.flag(S_IRQ_DISABLE, true);
    }
}
