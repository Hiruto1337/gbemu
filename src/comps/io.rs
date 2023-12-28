use std::sync::RwLock;

use super::{
    common::between,
    cpu::CPUContext,
    lcd::LCD,
    timer::{timer_read, timer_write},
};

static SERIAL_DATA: RwLock<[u8; 2]> = RwLock::new([0, 0]);

pub fn io_read(cpu: &CPUContext, address: u16) -> u8 {
    match address {
        0xFF01 => SERIAL_DATA.read().unwrap()[0],
        0xFF02 => SERIAL_DATA.read().unwrap()[1],
        addr if between(addr, 0xFF04, 0xFF07) => timer_read(address),
        0xFF0F => cpu.get_int_flags(),
        addr if between(addr, 0xFF40, 0xFF4B) => LCD.read().unwrap().read(address),
        _ => {
            println!("UNSUPPORTED: Bus.read({address:04X}): I/O Registers");
            0
        }
    }
}

pub fn io_write(cpu: &mut CPUContext, address: u16, value: u8) {
    match address {
        0xFF01 => SERIAL_DATA.write().unwrap()[0] = value,
        0xFF02 => SERIAL_DATA.write().unwrap()[1] = value,
        addr if between(addr, 0xFF04, 0xFF07) => timer_write(address, value),
        0xFF0F => cpu.set_int_flags(value),
        addr if between(addr, 0xFF40, 0xFF4B) => LCD.write().unwrap().write(address, value),
        _ => println!("UNSUPPORTED: Bus.write({address:04X}): I/O Registers"),
    }
}
