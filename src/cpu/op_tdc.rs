use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tdc(&mut self, opcode: u8) {
        let olda = self.reg_a.data;
        self.reg_a.data = self.reg_d;

        debug!(
            "[0x{:X}:0x{:X}] TDC : OLD_A=0x{:X} NEW_A=0x{:X}",
            self.pc, opcode, olda, self.reg_a.data,
        );
    }
}
