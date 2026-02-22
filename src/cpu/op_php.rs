use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_php(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.bus.write_byte(self.sp, self.reg_p);
        self.sp -= 1;
        debug!(
            "[0x{:X}] PHP : P=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_p, oldsp, self.sp
        );
    }
}
