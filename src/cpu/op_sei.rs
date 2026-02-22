use crate::cpu::alu::{Cpu, S_IRQ_DISABLE};
use log::debug;

impl Cpu {
    pub fn op_sei(&mut self, opcode: u8) {
        self.flag(S_IRQ_DISABLE, true);
        debug!(
            "[0x{:X}:0x{:X}] SEI : FLAGS={:08b}",
            self.pc, opcode, self.reg_p
        );
    }
}
