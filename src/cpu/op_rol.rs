use crate::cpu::alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY, S_CARRY};
use crate::cpu::bits::Word;
use log::debug;

impl Cpu {
    pub fn op_rol(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let oldpc = self.pc;
        let val = match opcode {
            0x2A => self.reg_a.data,
            0x26 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0x36 => self.fetch(AddressMode::ZeroPageX, sixteen_bits_mode),
            0x2E => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0x3E => self.fetch(AddressMode::AbsoluteIndexedX, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        let old_carry = self.reg_p & S_CARRY;
        let mut result = Word { data: 0x0 };
        let new_carry: bool;

        if sixteen_bits_mode {
            result.data = ((val << 1) & 0xFFFF) | old_carry as u16;
            new_carry = (val & 0x8000 >> 15) > 0;
        } else {
            result.data = ((val << 1) & 0xFF) | old_carry as u16;
            new_carry = (val & 0x80 >> 7) > 0;
        }

        self.flag_c(new_carry);
        self.flag_nz(result.data);

        match opcode {
            0x2A => self.reg_a = result.clone(),
            0x26 => self.store(AddressMode::ZeroPage, &result, sixteen_bits_mode),
            0x36 => self.store(AddressMode::ZeroPageX, &result, sixteen_bits_mode),
            0x2E => self.store(AddressMode::Absolute, &result, sixteen_bits_mode),
            0x3E => self.store(AddressMode::AbsoluteIndexedX, &result, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        debug!(
            "[0x{:X}:0x{:X}] ROL : RESULT=0x{:X} FLAGS={:b}",
            oldpc, opcode, result.data, self.reg_p
        );
    }
}
