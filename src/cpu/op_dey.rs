use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_dey(&mut self, opcode: u8) {
        self.reg_y -= 1;
        self.flag_nz(self.reg_y);
        debug!(
            "[0x{:X}] DEY : Y=0x{:X} FLAGS={:b}",
            opcode, self.reg_y, self.reg_p
        );
    }
}
