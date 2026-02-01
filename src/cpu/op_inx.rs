use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_inx(&mut self) {
        self.reg_x += 1;
        self.flag_nz(self.reg_x);
    }
}
