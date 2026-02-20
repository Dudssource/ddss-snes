use log::debug;

use crate::cpu::alu::{Cpu, S_DECIMAL_MODE};

impl Cpu {
    pub fn op_sed(&mut self, opcode: u8) {
        self.flag(S_DECIMAL_MODE, true);
        debug!("[0x{:X}] SED : FLAGS={:b}", opcode, self.reg_p);
    }
}
