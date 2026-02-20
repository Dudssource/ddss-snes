use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_txs(&mut self, opcode: u8) {
        self.sp = self.reg_x as u32;
        debug!("[0x{:X}:0x{:X}] TXS : SP=0x{:X}", self.pc, opcode, self.sp);
    }
}
