use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_txa(&mut self, opcode: u8) {
        let olda = self.reg_a;
        self.reg_a.data = self.reg_x;
        self.flag_nz(self.reg_a.data);

        debug!(
            "[0x{:X}:0x{:X}] TXA : OLD_A=0x{:X} NEW_A=0x{:X}",
            self.pc, opcode, olda, self.reg_a.data,
        );
    }
}
