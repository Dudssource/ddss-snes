use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_per(&mut self, opcode: u8) {
        let oldpc = self.pc;
        let oldsp = self.sp;

        self.incr_pc();
        let offset_lo = self.bus.read_byte(self.pbr_pc());

        self.incr_pc();
        let offset_hi = self.bus.read_byte(self.pbr_pc());

        let mut offset = Self::make_word(offset_lo, offset_hi);
        let mut pc = self.pc;

        if (offset & 0x8000) > 0 {
            offset = (!offset) + 1;
            pc -= offset;
        } else {
            pc += offset;
        }

        self.bus.write_byte(self.sp, ((pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        self.bus.write_byte(self.sp, (pc & 0xFF) as u8);
        self.sp -= 1;

        debug!(
            "[0x{:X}] PER : OLD_PC=0x{:X} PC=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, oldpc, pc, oldsp, self.sp
        );
    }
}
