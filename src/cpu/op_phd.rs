use log::debug;

use crate::cpu::alu::Cpu;
use crate::cpu::bits::Word;

impl Cpu {
    pub fn op_phd(&mut self, opcode: u8) {
        let value = Word { data: self.reg_d };
        let oldsp = self.sp;
        self.bus.write_byte(self.sp, value.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, value.lo());
        self.sp -= 1;
        debug!(
            "[0x{:X}] PHD : D=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_d, oldsp, self.sp
        );
    }
}
