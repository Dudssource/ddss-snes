use crate::cpu::alu::{Cpu, S_IRQ_DISABLE};

impl Cpu {
    pub fn op_sei(&mut self) {
        self.flag(S_IRQ_DISABLE, true);
    }
}
