use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_rti(&mut self, opcode: u8) {
        let oldpc = self.pc;
        let oldp = self.reg_p;
        let oldpb = self.reg_pb;

        // pop P
        self.sp += 1;
        let p = self.bus.read_byte(self.sp);

        // pop PC Low
        self.sp += 1;
        let pcl = self.bus.read_byte(self.sp);

        // pop PC High
        self.sp += 1;
        let pch = self.bus.read_byte(self.sp);

        // pop PBR
        self.sp += 1;
        let pbr = self.bus.read_byte(self.sp);

        // Save new PC
        let newpc = Self::make_word(pcl, pch);
        debug!(
            "[0x{:X}:0x{:X}] RTI : OLD_PC=0x{:X} NEW_PC=0x{:X} OLD_P=0x{:X} NEW_P=0x{:X} OLD_PBR=0x{:X} NEW_PBR=0x{:X}",
            oldpc, opcode, self.pc, newpc, oldp, p, oldpb, pbr,
        );
        self.pc = newpc;
        self.reg_p = p;
        self.reg_pb = pbr;
    }
}
