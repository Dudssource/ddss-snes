use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_jsl(&mut self) {
        // New PC Low
        self.pc += 1;
        let pcl = self.bus.read_byte(self.pbr_pc()) as u16;

        // New PC High
        self.pc += 1;
        let pch = self.bus.read_byte(self.pbr_pc()) as u16;

        // push PBR
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;

        // New PBR
        self.pc += 1;
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
        self.pc = (pch << 8) | pcl;
    }
}
