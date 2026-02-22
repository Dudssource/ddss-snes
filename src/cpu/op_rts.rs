use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_rts(&mut self, opcode: u8) {
        let oldpc = self.pc;

        // pop PC Low
        self.sp += 1;
        let pcl = self.bus.read_byte(self.sp);

        // pop PC High
        self.sp += 1;
        let pch = self.bus.read_byte(self.sp);

        // Save new PC
        let newpc = Self::make_word(pcl, pch);
        debug!(
            "[0x{:X}:0x{:X}] RTS : OLD_PC=0x{:X} NEW_PC=0x{:X}",
            oldpc, opcode, self.pc, newpc
        );
        self.pc = newpc;
    }
}
