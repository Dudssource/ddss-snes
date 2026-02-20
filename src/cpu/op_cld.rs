use log::debug;

use crate::cpu::alu::{Cpu, S_DECIMAL_MODE};

impl Cpu {
    pub fn op_cld(&mut self, opcode: u8) {
        self.flag(S_DECIMAL_MODE, false);
        debug!("[0x{:X}] CLD : FLAGS={:b}", opcode, self.reg_p);
    }
}
