use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_jml(&mut self) {
        self.pc += 1;
        let addr_lo = self.bus.read_byte(self.pbr_pc());

        self.pc += 1;
        let addr_hi = self.bus.read_byte(self.pbr_pc());

        let addr = Self::make_word(addr_lo, addr_hi) as u32;

        let pcl = self.bus.read_byte(addr) as u16;
        let pch = self.bus.read_byte(addr + 1) as u16;
        let pbr = self.bus.read_byte(addr + 2);

        // Save new PBR (bank)
        self.reg_pb = pbr;

        // Save new PC
        self.pc = (pch << 8) | pcl;
    }
}
