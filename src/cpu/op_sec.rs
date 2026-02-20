use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_sec(&mut self, opcode: u8) {
        self.flag_c(true);
        debug!("[0x{:X}] SEC : FLAGS={:b}", opcode, self.reg_p);
    }
}
