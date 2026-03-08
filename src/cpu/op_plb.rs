use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_plb(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.sp += 1;
        let reg_lo = self.bus.read_byte(self.sp);
        self.reg_db = reg_lo;

        debug!(
            "[0x{:X}] PLB : DBR=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_db, oldsp, self.sp
        );
    }
}
