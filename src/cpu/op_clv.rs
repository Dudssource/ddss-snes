use log::debug;

use crate::cpu::alu::{Cpu, S_OVERFLOW};

impl Cpu {
    pub fn op_clv(&mut self, opcode: u8) {
        self.flag(S_OVERFLOW, false);
        debug!("[0x{:X}] CLV : FLAGS={:b}", opcode, self.reg_p);
    }
}
