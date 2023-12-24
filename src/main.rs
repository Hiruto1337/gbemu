use std::time::Duration;

use gbemu::comps::{cart::CART, cpu::{CPUContext, Registers}, emu::EMULATOR, instructions::INSTRUCTIONS, timer::TIMER};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    CART.lock().unwrap().load(args);

    // Initialize SDL and fonts

    // Run CPU on separate thread
    let cpu_thread = std::thread::spawn(|| {
        let mut cpu = CPUContext {
            registers: Registers {
                pc: 0x100,
                sp: 0xFFFE,
                a: 0x01,
                f: 0xB0,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D
            },
            ie_register: 0,
            int_flags: 0,
            int_master_enabled: false,
            enabling_ime: false,
            
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: &INSTRUCTIONS[0],
            halted: false,
            stepping: true,
        };

        TIMER.lock().unwrap().div = 0xABCC;

        while EMULATOR.lock().unwrap().running {
            if EMULATOR.lock().unwrap().paused {
                delay(10);
                continue;
            }

            cpu.step();
        }
    });
    // When cpu_init() is run, the timer's div is set to 0xABCC?

    while !EMULATOR.lock().unwrap().die {
        delay(1);
        // Handle UI events
    }
}

pub fn delay(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
