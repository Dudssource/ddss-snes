use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_jmp(&mut self, opcode: u8) {
        // self.incr_pc();

        let oldpc = self.pc;
        self.pc = match opcode {
            // Absolute
            0x4C => {
                let pcl = self.bus.read_byte(self.pbr_pc()) as u16;
                self.incr_pc();
                let pch = self.bus.read_byte(self.pbr_pc()) as u16;

                (pch << 8) | pcl
            }

            // Absolute Indirect
            0x6C => {
                let addr_lo = self.bus.read_byte(self.pbr_pc());
                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());
                let addr = Self::make_word(addr_lo, addr_hi) as u32;
                let pcl = self.bus.read_byte(addr) as u16;
                let pch = self.bus.read_byte(addr + 1) as u16;

                (pch << 8) | pcl
            }

            // Absolute Indexed Indirect
            0x7C => {
                let addr_lo = self.bus.read_byte(self.pbr_pc());
                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());
                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;
                let pcl = self.bus.read_byte(addr) as u16;
                let pch = self.bus.read_byte(addr + 1) as u16;

                (pch << 8) | pcl
            }

            // Absolute Long
            0x5C => {
                let pcl = self.bus.read_byte(self.pbr_pc()) as u16;
                self.incr_pc();
                let pch = self.bus.read_byte(self.pbr_pc()) as u16;
                self.incr_pc();
                let pbr = self.bus.read_byte(self.pbr_pc());

                // Save new PBR (bank)
                self.reg_pb = pbr;

                (pch << 8) | pcl
            }
            
            _ => panic!("invalid opcode {}", opcode),
        };

        debug!(
            "[0x{:X}] JMP : OLD_PC=0x{:X} NEW_PC=0x{:X} PB=0x{:X}",
            opcode, oldpc, self.pc, self.reg_pb
        );
    }
}
