use log::debug;

use crate::cpu::alu::{AddressMode, Cpu};

impl Cpu {
    pub fn op_pea(&mut self, opcode: u8) {
        let oldsp = self.sp;

        // always pushes sixteen bits of data, irrespective of the settings of the m and x mode select flag
        let value = self.fetch(AddressMode::Immediate, true);

        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
        self.sp -= 1;

        debug!(
            "[0x{:X}] PEA : VALUE=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, value, oldsp, self.sp
        );
    }
}
