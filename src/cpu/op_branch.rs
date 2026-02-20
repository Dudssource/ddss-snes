use log::debug;

use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_branch(&mut self, opcode: u8) {
        let taken: Result<bool, String> = match opcode {
            // BNE
            0xD0 => Ok((self.reg_p & S_RESULT_ZERO) == 0),

            // BEQ
            0xF0 => Ok((self.reg_p & S_RESULT_ZERO) > 0),

            // BMI
            0x30 => Ok((self.reg_p & S_NEGATIVE) > 0),

            // BPL
            0x10 => Ok((self.reg_p & S_NEGATIVE) == 0),

            // BCS
            0xB0 => Ok((self.reg_p & S_CARRY) > 0),

            // BCC
            0x90 => Ok((self.reg_p & S_CARRY) == 0),

            // BVC
            0x50 => Ok((self.reg_p & S_OVERFLOW) == 0),

            // BVS
            0x70 => Ok((self.reg_p & S_OVERFLOW) > 0),

            // BRA
            0x80 => Ok(true),

            _ => Err(format!("branch : unknown opcode {}", opcode)),
        };

        match taken {
            Ok(taken) => {
                //self.incr_pc();
                let offset = self.bus.read_byte(self.pbr_pc());

                // TODO: Add 1 more cycle if branch is taken
                if taken {
                    // if signed, flip all bits and add 1 to get real value, then subtract from PC
                    // this is because offset is a one byte signed two's-complement
                    self.pc = match (offset & 0x80) > 0 {
                        true => self.pc - ((!offset) + 1) as u16,
                        false => self.pc + offset as u16,
                    };

                    debug!(
                        "[0x{:X}] BRANCH : OFFSET=0x{:X} PC=0x{:X} FLAGS={:b}",
                        opcode, offset, self.pc, self.reg_p
                    );
                } else {
                    debug!(
                        "[0x{:X}] BRANCH : OFFSET=0x{:X} FLAGS={:b}",
                        opcode, offset, self.reg_p
                    );
                }
            }
            Err(msg) => panic!("error : {}", msg),
        }
    }
}
