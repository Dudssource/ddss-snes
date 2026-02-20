use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_phk(&mut self, opcode: u8) {
        // self.incr_pc();
        let oldsp = self.sp;
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;
        debug!(
            "[0x{:X}] PHK : PB=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_pb, oldsp, self.sp
        );
    }
}
