use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_rep(&mut self, opcode: u8) {
        let oldpc = self.pc;
        self.incr_pc();
        let mut mask = self.bus.read_byte(self.pbr_pc());
        if self.emulation {
            mask &= 0xCF;
        }
        self.reg_p = self.reg_p & (!mask);
        debug!(
            "[0x{:X}:0x{:X}] REP : MASK={:X} FLAGS={:b}",
            oldpc, opcode, mask, self.reg_p
        );
    }
}
