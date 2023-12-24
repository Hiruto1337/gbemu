use std::sync::Mutex;

use super::{cpu::CPUContext, bus::{bus_read, bus_write}};

static DBG_MSG: Mutex<[char; 1024]> = Mutex::new([' '; 1024]);
static MSG_SIZE: Mutex<usize> = Mutex::new(0);

pub fn dbg_update(cpu: &mut CPUContext) {
    if bus_read(cpu, 0xFF02) == 0x81 {
        let c = bus_read(cpu, 0xFF01) as char;
        DBG_MSG.lock().unwrap()[*MSG_SIZE.lock().unwrap()] = c;
        *MSG_SIZE.lock().unwrap() += 1;

        bus_write(cpu, 0xFF02, 0);
    }
}
pub fn dbg_print() {
    if DBG_MSG.lock().unwrap()[0] != ' ' {
        println!("DBG: {:?}", DBG_MSG.lock().unwrap()[0..*MSG_SIZE.lock().unwrap()].iter().collect::<String>());
    }
}