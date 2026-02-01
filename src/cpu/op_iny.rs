use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_iny(&mut self) {
        self.reg_y += 1;
        self.flag_nz(self.reg_y);
    }
}
