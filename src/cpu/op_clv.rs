use crate::cpu::alu::{Cpu, S_OVERFLOW};

impl Cpu {
    pub fn op_clv(&mut self) {
        self.flag(S_OVERFLOW, false);
    }
}
