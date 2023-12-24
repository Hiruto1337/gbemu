use std::sync::Mutex;

use super::{
    common::between,
    cpu::CPUContext,
    timer::{timer_read, timer_write},
};

static SERIAL_DATA: Mutex<[u8; 2]> = Mutex::new([0, 0]);

pub fn io_read(cpu: &mut CPUContext, address: u16) -> u8 {
    match address {
        0xFF01 => return SERIAL_DATA.lock().unwrap()[0],
        0xFF02 => return SERIAL_DATA.lock().unwrap()[1],
        val if between(val, 0xFF04, 0xFF07) => return timer_read(address),
        0xFF0F => cpu.get_int_flags(),
        _ => {
            println!("UNSUPPORTED: Bus.read({address:04X}): I/O Registers");
            0
        }
    }
}

pub fn io_write(cpu: &mut CPUContext, address: u16, value: u8) {
    match address {
        0xFF01 => SERIAL_DATA.lock().unwrap()[0] = value,
        0xFF02 => SERIAL_DATA.lock().unwrap()[1] = value,
        val if between(val, 0xFF04, 0xFF07) => timer_write(address, value),
        0xFF0F => cpu.set_int_flags(value),
        _ => println!("UNSUPPORTED: Bus.write({address:04X}): I/O Registers"),
    }
}
