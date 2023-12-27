use std::sync::RwLock;

use super::{
    common::between,
    cpu::CPUContext,
    dma::DMA,
    timer::{timer_read, timer_write},
};

static SERIAL_DATA: RwLock<[u8; 2]> = RwLock::new([0, 0]);
static LY: RwLock<u8> = RwLock::new(0);

pub fn io_read(cpu: &CPUContext, address: u16) -> u8 {
    match address {
        0xFF01 => return SERIAL_DATA.read().unwrap()[0],
        0xFF02 => return SERIAL_DATA.read().unwrap()[1],
        val if between(val, 0xFF04, 0xFF07) => return timer_read(address),
        0xFF0F => cpu.get_int_flags(),
        0xFF44 => {
            // NOTICE: Might be incorrect
            let mut ly = LY.write().unwrap();
            *ly = ly.wrapping_add(1);
            ly.wrapping_sub(1)
        }
        _ => {
            // println!("UNSUPPORTED: Bus.read({address:04X}): I/O Registers");
            0
        }
    }
}

pub fn io_write(cpu: &mut CPUContext, address: u16, value: u8) {
    match address {
        0xFF01 => SERIAL_DATA.write().unwrap()[0] = value,
        0xFF02 => SERIAL_DATA.write().unwrap()[1] = value,
        val if between(val, 0xFF04, 0xFF07) => timer_write(address, value),
        0xFF0F => cpu.set_int_flags(value),
        0xFF46 => {
            DMA.write().unwrap().start(value);
            println!("DMA START!");
        }
        _ =>
            /*println!("UNSUPPORTED: Bus.write({address:04X}): I/O Registers")*/
            {}
    }
}
