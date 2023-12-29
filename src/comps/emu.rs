use std::sync::RwLock;

use super::{timer::timer_tick, cpu::CPUContext, dma::DMA, ppu::{PPU, PPUContext}};

/*
    Emu components:

    |Cart|
    |CPU|
    |Address bus| // Maps a memory address to data location
    |PPU|
    |Timer|
*/

/*
            |CPU| -> |Bus| -> |Cart|
              ^
        <- |Emulator|
*/

pub struct EmulatorContext {
    pub running: bool,
    pub paused: bool,
    pub die: bool,
    pub ticks: u64,
}

pub static EMULATOR: RwLock<EmulatorContext> = RwLock::new(EmulatorContext {
    running: true,
    paused: false,
    die: false,
    ticks: 0,
});

impl EmulatorContext {
    pub fn cycles(&mut self, cpu: &mut CPUContext, ppu: &mut PPUContext, cpu_cycles: u8) {
        for _ in 0..cpu_cycles {
            for _ in 0..4 {
                self.ticks += 1;
                timer_tick(cpu);
                ppu.tick(cpu);
            }

            DMA.write().unwrap().tick(cpu, ppu);
        }
    }
}
