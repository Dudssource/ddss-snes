use log::debug;

use crate::cpu::{
    alu::{AddressMode, Cpu},
    bits::Word,
};

impl Cpu {
    pub fn op_dec(&mut self, opcode: u8) {
        let mut value = match opcode {
            0xC6 => self.fetch(AddressMode::ZeroPage, true),
            0xD6 => self.fetch(AddressMode::ZeroPageX, true),
            0xCE => self.fetch(AddressMode::Absolute, true),
            0xDE => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0x3A => self.reg_a.data,
            _ => panic!("invalid opcode {}", opcode),
        };

        // dec
        if self.emulation {
            value = (value & 0xFF) - 1
        } else {
            value -= 1
        };

        // NZ
        self.flag_nz(value);

        // store
        match opcode {
            0xC6 => self.store(AddressMode::ZeroPage, &Word { data: value }, true),
            0xD6 => self.store(AddressMode::ZeroPageX, &Word { data: value }, true),
            0xCE => self.store(AddressMode::Absolute, &Word { data: value }, true),
            0xDE => self.store(AddressMode::AbsoluteIndexedX, &Word { data: value }, true),
            0x3A => self.reg_a.data = value,
            _ => panic!("invalid opcode {}", opcode),
        };

        debug!(
            "[0x{:X}] DEC : VALUE=0x{:X} FLAGS={:b}",
            opcode, value, self.reg_p
        );
    }
}
