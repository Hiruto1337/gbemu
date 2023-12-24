use std::sync::Mutex;

use super::{timer::timer_tick, cpu::CPUContext};

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

pub static EMULATOR: Mutex<EmulatorContext> = Mutex::new(EmulatorContext {
    running: true,
    paused: false,
    die: false,
    ticks: 0,
});

impl EmulatorContext {
    pub fn cycles(&mut self, cpu: &mut CPUContext, cpu_cycles: u8) {
        let n = cpu_cycles as usize * 4;

        for _ in 0..n {
            self.ticks += 1;
            timer_tick(cpu);
        }
    }
}
