use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_phk(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;
    }
}