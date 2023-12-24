use std::sync::Mutex;

pub struct RAMContext {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

pub static RAM: Mutex<RAMContext> = Mutex::new(RAMContext {
    wram: [0; 0x2000],
    hram: [0; 0x80],
});

impl RAMContext {
    pub fn wram_read(&self, mut address: u16) -> u8 {
        address -= 0xC000;

        if 0x2000 <= address {
            panic!("Invalid WRAM address: {:08X}\n", address + 0xC000);
        }

        self.wram[address as usize]
    }

    pub fn wram_write(&mut self, mut address: u16, value: u8) {
        address -= 0xC000;

        self.wram[address as usize] = value;
    }

    pub fn hram_read(&self, mut address: u16) -> u8 {
        address -= 0xFF80;

        self.hram[address as usize]
    }

    pub fn hram_write(&mut self, mut address: u16, value: u8) {
        address -= 0xFF80;

        self.hram[address as usize] = value;
    }
}
