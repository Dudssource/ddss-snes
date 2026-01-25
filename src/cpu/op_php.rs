use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_php(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_p);
        self.sp -= 1;
    }
}