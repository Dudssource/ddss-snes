use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_pha(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_a.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, self.reg_a.lo());
        self.sp -= 1;
    }
}
