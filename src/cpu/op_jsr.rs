use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_jsr(&mut self) {
        // New PC Low
        self.pc += 1;
        let pcl = self.bus.read_byte(self.pbr_pc());

        // New PC High
        self.pc += 1;
        let pch = self.bus.read_byte(self.pbr_pc());

        // push PC High
        self.bus
            .write_byte(self.sp, ((self.pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        // push PC Low
        self.bus.write_byte(self.sp, (self.pc & 0xFF) as u8);
        self.sp -= 1;

        // Save new PC
        self.pc = Self::make_word(pcl, pch);
    }
}
