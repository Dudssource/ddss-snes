use crate::cpu::alu::{Cpu, S_DECIMAL_MODE};

impl Cpu {
    pub fn op_sed(&mut self) {
        self.flag(S_DECIMAL_MODE, true);
    }
}
