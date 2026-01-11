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

    pub fn write_byte(&mut self, val: u8, addr: u32) {
        match addr {
            _ => self.work_ram[addr as usize] = val,
        }
    }

    pub fn read_dword(&self, addr: u32) -> u32 {
        self.read_byte(addr) as u32
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            _ => self.work_ram[addr as usize],
        }
    }
}
