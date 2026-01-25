use crate::cpu::alu::Cpu;
use crate::cpu::bits::Word;

impl Cpu {
    pub fn op_phd(&mut self) {
        let value = Word { data: self.reg_d };
        self.pc += 1;
        self.bus.write_byte(self.sp, value.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, value.lo());
        self.sp -= 1;
    }
}
