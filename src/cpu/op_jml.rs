use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_jml(&mut self, opcode: u8) {
        // self.incr_pc();
        let addr_lo = self.bus.read_byte(self.pbr_pc());

        self.incr_pc();
        let addr_hi = self.bus.read_byte(self.pbr_pc());

        let addr = Self::make_word(addr_lo, addr_hi) as u32;

        let pcl = self.bus.read_byte(addr) as u16;
        let pch = self.bus.read_byte(addr + 1) as u16;
        let pbr = self.bus.read_byte(addr + 2);

        // Save new PBR (bank)
        self.reg_pb = pbr;

        // Save new PC
        let newpc = (pch << 8) | pcl;
        debug!(
            "[0x{:X}] JML : OLD_PC=0x{:X} NEW_PC=0x{:X} PB=0x{:X}",
            opcode, self.pc, newpc, self.reg_pb
        );
        self.pc = newpc;
    }
}
