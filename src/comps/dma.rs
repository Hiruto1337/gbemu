// Direct Memory Access

use std::sync::Mutex;

use super::{ppu::PPU, bus::bus_read};

pub struct DMAContext {
    active: bool,
    byte: u8,
    value: u8,
    start_delay: u8
}

pub static DMA: Mutex<DMAContext> = Mutex::new(DMAContext {
    active: false,
    byte: 0,
    value: 0,
    start_delay: 0
});

impl DMAContext {
    pub fn dma_start(&mut self, start: u8) {
        self.active = true;
        self.byte = 0;
        self.start_delay = 2;
        self.value = start;
    }
    
    pub fn dma_tick(&mut self) {
        if !self.active {
            return;
        }

        if self.start_delay != 0 {
            self.start_delay -= 1;
            return;
        }

        let ppu = PPU.lock().unwrap();

        // ppu.oam_write(self.byte as u16, bus_read(cpu, address))
    }
    
    // pub fn dma_transferring() -> bool {}
}