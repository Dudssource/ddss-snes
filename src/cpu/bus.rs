const STACK_POINTER_START: u16 = 0x1FF;
const STACK_POINTER_END: u16 = 0x100;

pub struct Bus {
    work_ram: Box<[u8; 0xFFFF]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            work_ram: Box::new([0; 0xFFFF]),
        }
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) {
        match addr {
            _ => self.work_ram[addr as usize] = val,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            _ => self.work_ram[addr as usize],
        }
    }

    pub fn r16(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16 | ((self.read_byte(addr + 1) as u16) << 8)) as u16
    }
}
