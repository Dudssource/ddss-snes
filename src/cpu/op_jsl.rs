use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_jsl(&mut self, opcode: u8) {
        // New PC Low
        // self.incr_pc();
        let pcl = self.bus.read_byte(self.pbr_pc()) as u16;

        // New PC High
        self.incr_pc();
        let pch = self.bus.read_byte(self.pbr_pc()) as u16;

        // push PBR
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;

        // New PBR
        self.incr_pc();
        let pbr = self.bus.read_byte(self.pbr_pc());

        // push PC High
        self.bus
            .write_byte(self.sp, ((self.pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        // push PC Low
        self.bus.write_byte(self.sp, (self.pc & 0xFF) as u8);
        self.sp -= 1;

        // Save new PBR (bank)
        self.reg_pb = pbr;

        // Save new PC
        let newpc = (pch << 8) | pcl;
        debug!(
            "[0x{:X}] JSL : OLD_PC=0x{:X} NEW_PC=0x{:X} PB=0x{:X}",
            opcode, self.pc, newpc, self.reg_pb
        );
        self.pc = newpc;
    }
}
