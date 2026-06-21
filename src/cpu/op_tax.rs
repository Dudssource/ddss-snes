use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tax(&mut self, opcode: u8) {
        let oldx = self.reg_x;
        self.reg_x = self.reg_a.data;
        self.flag_nz(self.reg_x);

        debug!(
            "[0x{:X}:0x{:X}] TAX : OLD_X=0x{:X} REG_A=0x{:X}",
            self.pc, opcode, oldx, self.reg_a.data,
        );
    }
}
