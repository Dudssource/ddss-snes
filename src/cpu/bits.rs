#[derive(Debug, Clone)]
pub struct Word {
    pub data: u16,
}

impl Word {
    pub fn new(hi: u8, lo: u8) -> Self {
        let val = ((hi as u16) << 8) | lo as u16;
        Self { data: val }
    }

    pub fn hi(&self) -> u8 {
        ((self.data & 0xFF00u16) >> 8) as u8
    }

    pub fn lo(&self) -> u8 {
        (self.data & 0xFFu16) as u8
    }
}
