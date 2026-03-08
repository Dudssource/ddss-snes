use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_plp(&mut self, opcode: u8) {
        let oldsp = self.sp;
        self.sp += 1;
        let mut reg_lo = self.bus.read_byte(self.sp);

        // the e flag–the 6502 emulation mode flag on the 65802/65816–is not on the stack so
        // cannot be pulled from it
        if self.emulation {
            reg_lo &= 0xDF;
        }

        self.reg_p = reg_lo;

        debug!(
            "[0x{:X}] PLP : FLAGS={:b} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_p, oldsp, self.sp
        );
    }
}
