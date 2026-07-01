use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tcd(&mut self, opcode: u8) {
        let oldd = self.reg_d;
        self.reg_d = self.reg_a.data;
        self.flag_nz(self.reg_d);

        debug!(
            "[0x{:X}:0x{:X}] TCD : OLD_D=0x{:X} NEW_D=0x{:X}",
            self.pc, opcode, oldd, self.reg_d,
        );
    }
}
