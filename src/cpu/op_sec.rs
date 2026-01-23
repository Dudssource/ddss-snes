use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_sec(&mut self) {
        self.flag_c(true);
    }
}
