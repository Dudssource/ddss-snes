use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_dex(&mut self, opcode: u8) {
        self.reg_x -= 1;
        self.flag_nz(self.reg_x);
        debug!(
            "[0x{:X}] DEX : X=0x{:X} FLAGS={:b}",
            opcode, self.reg_x, self.reg_p
        );
    }
}
