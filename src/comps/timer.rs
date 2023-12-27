use std::sync::RwLock;

use super::{cpu::CPUContext, interrupts::InterruptType};

pub struct TimerContext {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8
}

// GLOBAL TIMER
pub static TIMER: RwLock<TimerContext> = RwLock::new(TimerContext {
    div: 0xAC00,
    tima: 0,
    tma: 0,
    tac: 0
});

pub fn timer_tick(cpu: &mut CPUContext) {
    let timer = &mut TIMER.write().unwrap();
    let prev_div = timer.div;

    timer.div = timer.div.wrapping_add(1);

    let timer_update;

    match timer.tac & 0b11 {
        0b00 => timer_update = (prev_div & (1 << 9) != 0) && (timer.div & (1 << 9) == 0),
        0b01 => timer_update = (prev_div & (1 << 3) != 0) && (timer.div & (1 << 3) == 0),
        0b10 => timer_update = (prev_div & (1 << 5) != 0) && (timer.div & (1 << 5) == 0),
        0b11 => timer_update = (prev_div & (1 << 7) != 0) && (timer.div & (1 << 7) == 0),
        _ => unreachable!()
    }

    if timer_update && timer.tac & (1 << 2) != 0 { // NOTICE: Rewritten to fit Pan Docs
        match timer.tima.checked_add(1) {
            Some(result) => timer.tima = result,
            None => {
                timer.tima = timer.tma;
                cpu.request_interrupt(InterruptType::Timer);
            }
        }
    }
}

pub fn timer_write(address: u16, value: u8) {
    let mut timer = TIMER.write().unwrap();

    match address {
        0xFF04 => timer.div = 0,
        0xFF05 => timer.tima = value,
        0xFF06 => timer.tma = value,
        0xFF07 => timer.tac = value,
        _ => panic!("Invalid time address!")
    }
}

pub fn timer_read(address: u16) -> u8 {
    let timer = TIMER.read().unwrap();
    
    match address {
        0xFF04 => (timer.div >> 8) as u8,
        0xFF05 => timer.tima,
        0xFF06 => timer.tma,
        0xFF07 => timer.tac,
        _ => panic!("Invalid time address!")
    }
}