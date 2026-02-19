use crate::cpu::alu::Cpu;
use log::debug;

impl Cpu {
    pub fn op_xce(&mut self) {

        debug!("XCE : PC 0x{:X}", self.pc);

        // c takes emulation’s pervious value: set if previous mode was emulation; else cleared
        self.flag_c(self.emulation);

        // enable/disable emulation
        self.emulation = !self.emulation;

        // m is a native mode flag only; switching to native mode sets it to 1
        self.flag(0x20, !self.emulation);

        // x is a native mode flag only; it becomes the b flag in emulation.
        // b is an emulation mode flag only; it is set to 1 to become the x flag in native.
        self.flag(0x10, !self.emulation);
    }
}
