use log::debug;

use crate::cpu::alu::{Cpu, S_IRQ_DISABLE};

impl Cpu {
    pub fn op_cli(&mut self, opcode: u8) {
        self.flag(S_IRQ_DISABLE, false);
        debug!("[0x{:X}] CLI : FLAGS={:b}", opcode, self.reg_p);
    }
}
