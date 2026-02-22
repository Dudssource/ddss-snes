use crate::cpu::alu::{Cpu, S_ACCUMULATOR_MEMORY, S_BREAK_INSTRUCTION, S_CARRY};
use log::debug;

impl Cpu {
    pub fn op_xce(&mut self, opcode: u8) {
        // save carry
        let carry = self.reg_p & S_CARRY > 0;

        // c takes emulation’s pervious value: set if previous mode was emulation; else cleared
        self.flag_c(self.emulation);

        // enable/disable emulation
        self.emulation = carry;

        // m is a native mode flag only; switching to native mode sets it to 1
        self.flag(S_ACCUMULATOR_MEMORY, !self.emulation);

        // x is a native mode flag only; it becomes the b flag in emulation.
        // b is an emulation mode flag only; it is set to 1 to become the x flag in native.
        self.flag(S_BREAK_INSTRUCTION, !self.emulation);

        debug!(
            "[0x{:X}:0x{:X}] XCE : EMULATION={} FLAGS={:08b}",
            self.pc, opcode, self.emulation, self.reg_p
        );
    }
}
