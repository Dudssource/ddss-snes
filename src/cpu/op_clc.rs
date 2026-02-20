use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_clc(&mut self, opcode: u8) {
        self.flag_c(false);
        debug!(
            "[0x{:X}:0x{:X}] CLC : FLAGS={:b}",
            self.pc, opcode, self.reg_p
        );
    }
}
