use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_pha(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.bus.write_byte(self.sp, self.reg_a.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, self.reg_a.lo());
        self.sp -= 1;
        debug!(
            "[0x{:X}] PHA : A=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_a.data, oldsp, self.sp
        );
    }
}
