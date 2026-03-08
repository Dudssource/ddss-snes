use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_pld(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.sp += 1;
        let reg_lo = self.bus.read_byte(self.sp);
        self.sp += 1;
        let reg_hi = self.bus.read_byte(self.sp);
        self.reg_d = Self::make_word(reg_lo, reg_hi);

        debug!(
            "[0x{:X}] PLD : D=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_d, oldsp, self.sp
        );
    }
}
