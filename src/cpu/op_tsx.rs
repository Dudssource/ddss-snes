use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tsx(&mut self, opcode: u8) {
        let oldx = self.sp;
        self.reg_x = self.sp as u16;
        self.flag_nz(self.reg_x);

        debug!(
            "[0x{:X}:0x{:X}] TSX : OLD_X=0x{:X} SP=0x{:X}",
            self.pc, opcode, oldx, self.sp as u16,
        );
    }
}
