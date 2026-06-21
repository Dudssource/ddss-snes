use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tay(&mut self, opcode: u8) {
        let oldy = self.reg_y;
        self.reg_y = self.reg_a.data;
        self.flag_nz(self.reg_y);

        debug!(
            "[0x{:X}:0x{:X}] TAY : OLD_Y=0x{:X} REG_A=0x{:X}",
            self.pc, opcode, oldy, self.reg_a.data,
        );
    }
}
