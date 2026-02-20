use log::debug;

use crate::cpu::alu::Cpu;

impl Cpu {
    pub fn op_phb(&mut self,opcode:u8) {
        // self.incr_pc();
        let oldsp=self.sp;
        self.bus.write_byte(self.sp, self.reg_db);
        self.sp -= 1;
        debug!(
            "[0x{:X}] PHB : DB=0x{:X} OLD_SP=0x{:X} NEW_SP=0x{:X}",
            opcode, self.reg_db, oldsp, self.sp
        );
    }
}
