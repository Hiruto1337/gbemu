use std::sync::RwLock;

use super::{cpu::CPUContext, bus::{bus_read, bus_write}};

static DBG_MSG: RwLock<[char; 1024]> = RwLock::new([' '; 1024]);
static MSG_SIZE: RwLock<usize> = RwLock::new(0);

pub fn dbg_update(cpu: &mut CPUContext) {
    if bus_read(cpu, 0xFF02) == 0x81 {
        let c = bus_read(cpu, 0xFF01) as char;
        DBG_MSG.write().unwrap()[*MSG_SIZE.read().unwrap()] = c;
        *MSG_SIZE.write().unwrap() += 1;

        bus_write(cpu, 0xFF02, 0);
    }
}
pub fn dbg_print() {
    if DBG_MSG.read().unwrap()[0] != ' ' {
        println!("DBG: {:?}", DBG_MSG.read().unwrap()[0..*MSG_SIZE.read().unwrap()].iter().collect::<String>());
    }
}