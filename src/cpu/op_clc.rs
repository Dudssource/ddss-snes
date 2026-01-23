use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_clc(&mut self) {
        self.flag_c(false);
    }
}
