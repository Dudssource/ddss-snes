use log::debug;

use crate::cpu::alu::{Cpu, S_ACCUMULATOR_MEMORY};

impl Cpu {
    pub fn op_ply(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;

        let oldsp = self.sp;
        self.sp += 1;
        let reg_lo = self.bus.read_byte(self.sp);
        let mut reg_hi = 0x0;

        if sixteen_bits_mode {
            self.sp += 1;
            reg_hi = self.bus.read_byte(self.sp);
        }

        self.reg_y = Self::make_word(reg_lo, reg_hi);
        self.flag_nz(self.reg_y);

        debug!(
            "[0x{:X}] PLY : Y=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_y, oldsp, self.sp
        );
    }
}
