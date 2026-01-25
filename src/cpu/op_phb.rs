use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_phb(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_db);
        self.sp -= 1;
    }
}
