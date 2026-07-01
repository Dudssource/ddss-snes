use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_tcs(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.sp = self.reg_a.data as u32;

        debug!(
            "[0x{:X}:0x{:X}] TCS : OLD_SP=0x{:X} NEW_SP=0x{:X}",
            self.pc, opcode, oldsp, self.sp,
        );
    }
}
