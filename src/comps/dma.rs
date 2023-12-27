// Direct Memory Access

use std::sync::RwLock;

use crate::comps::common::delay;

use super::{ppu::PPU, cpu::CPUContext, bus::bus_read};

pub struct DMAContext {
    pub active: bool,
    pub byte: u8,
    pub value: u8,
    pub start_delay: u8
}

pub static DMA: RwLock<DMAContext> = RwLock::new(DMAContext {
    active: false,
    byte: 0,
    value: 0,
    start_delay: 0
});

impl DMAContext {
    pub fn start(&mut self, start: u8) {
        self.active = true;
        self.byte = 0;
        self.start_delay = 2;
        self.value = start;
    }
    
    pub fn tick(&mut self, cpu: &CPUContext) {
        if !self.active {
            return;
        }

        if self.start_delay != 0 {
            self.start_delay -= 1;
            return;
        }

        // NOTICE: Should check
        let value = bus_read(cpu, self.value as u16 * 0x100 + self.byte as u16);
        PPU.write().unwrap().oam_write(self.byte as u16, value);
        self.byte += 1;

        self.active = self.byte < 0xA0;

        if !self.active {
            println!("DMA DONE!");
            delay(2000);
        }
    }
    
    pub fn transferring(&self) -> bool {
        self.active
    }
}