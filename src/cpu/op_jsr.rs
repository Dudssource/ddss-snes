use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_jsr(&mut self, opcode: u8) {
        let oldpc = self.pc;
        // New PC Low
        self.incr_pc();
        let pcl = self.bus.read_byte(self.pbr_pc());

        // New PC High
        self.incr_pc();
        let pch = self.bus.read_byte(self.pbr_pc());

        // push PC High
        self.bus
            .write_byte(self.sp, ((self.pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        // push PC Low
        self.bus.write_byte(self.sp, (self.pc & 0xFF) as u8);
        self.sp -= 1;

        // Save new PC
        let newpc = Self::make_word(pcl, pch);
        debug!(
            "[0x{:X}:0x{:X}] JSR : OLD_PC=0x{:X} NEW_PC=0x{:X}",
            oldpc, opcode, self.pc, newpc
        );
        self.pc = newpc - 1;
    }
}
