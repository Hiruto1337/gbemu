use crate::comps::{instructions::{inst_by_opcode, AddrMode, RegType}, cpu_util::{read_reg, set_reg}, emu::EMULATOR};

use super::{cpu::CPUContext, bus::bus_read};

pub fn fetch_instruction(cpu: &mut CPUContext) {
    cpu.cur_opcode = bus_read(cpu, cpu.registers.pc);
    cpu.registers.pc += 1;
    cpu.cur_inst = inst_by_opcode(cpu.cur_opcode);
}

pub fn fetch_data(cpu: &mut CPUContext) {
    cpu.mem_dest = 0;
    cpu.dest_is_mem = false;

    type AM = AddrMode;
    match cpu.cur_inst.mode {
        AM::IMP => {},
        AM::R => {
            cpu.fetched_data = read_reg(cpu, cpu.cur_inst.reg1);
        },
        AM::RxR => {
            cpu.fetched_data = read_reg(cpu, cpu.cur_inst.reg2);
        },
        AM::RxD8 => {
            cpu.fetched_data = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
        },
        AM::D16 | AM::RxD16 => {
            let lo = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);

            let hi = bus_read(cpu, cpu.registers.pc + 1) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            
            cpu.fetched_data = (hi << 8) | lo;
            cpu.registers.pc += 2;
        },
        AM::MRxR => {
            cpu.fetched_data = read_reg(cpu, cpu.cur_inst.reg2);
            cpu.mem_dest = read_reg(cpu, cpu.cur_inst.reg1);
            cpu.dest_is_mem = true;

            if cpu.cur_inst.reg1.unwrap() == RegType::C {
                cpu.mem_dest |= 0xFF00;
            }
        },
        AM::RxMR => {
            let mut addr = read_reg(cpu, cpu.cur_inst.reg2);

            if cpu.cur_inst.reg1.unwrap() == RegType::C {
                addr |= 0xFF00;
            }

            cpu.fetched_data = bus_read(cpu, addr) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
        },
        AM::RxHLI | AM::RxHLD => {
            cpu.fetched_data = bus_read(cpu, read_reg(cpu, cpu.cur_inst.reg2)) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);

            if cpu.cur_inst.mode == AM::RxHLI {
                set_reg(cpu, Some(RegType::HL), read_reg(cpu, Some(RegType::HL)) + 1);
            } else {
                set_reg(cpu, Some(RegType::HL), read_reg(cpu, Some(RegType::HL)) - 1)
            }
        },
        AM::HLIxR | AM::HLDxR => {
            cpu.fetched_data = read_reg(cpu, cpu.cur_inst.reg2);
            cpu.mem_dest = read_reg(cpu, cpu.cur_inst.reg1);
            cpu.dest_is_mem = true;

            if cpu.cur_inst.mode == AM::HLIxR {
                set_reg(cpu, Some(RegType::HL), read_reg(cpu, Some(RegType::HL)) + 1);
            } else {
                set_reg(cpu, Some(RegType::HL), read_reg(cpu, Some(RegType::HL)) - 1)
            }
        },
        AM::RxA8 => {
            cpu.fetched_data = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
        },
        AM::A8xR => {
            cpu.mem_dest = bus_read(cpu, cpu.registers.pc) as u16 | 0xFF00;
            cpu.dest_is_mem = true;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
        },
        AM::HLxSPR => {
            cpu.fetched_data = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
        },
        AM::D8 => {
            cpu.fetched_data = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
        },
        AM::D16xR | AM::A16xR => {
            let lo = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);

            let hi = bus_read(cpu, cpu.registers.pc + 1) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            
            cpu.mem_dest = (hi << 8) | lo;
            cpu.dest_is_mem = true;

            cpu.registers.pc += 2;
            cpu.fetched_data = read_reg(cpu, cpu.cur_inst.reg2);
        },
        AM::MRxD8 => {
            cpu.fetched_data = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            cpu.registers.pc += 1;
            cpu.mem_dest = read_reg(cpu, cpu.cur_inst.reg1);
            cpu.dest_is_mem = true;
        },
        AM::MR => {
            cpu.mem_dest = read_reg(cpu, cpu.cur_inst.reg1) as u16;
            cpu.dest_is_mem = true;
            cpu.fetched_data = bus_read(cpu, read_reg(cpu, cpu.cur_inst.reg1)) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
        },
        AM::RxA16 => {
            let lo = bus_read(cpu, cpu.registers.pc) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);

            let hi = bus_read(cpu, cpu.registers.pc + 1) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);

            let addr = (hi << 8) | lo;

            cpu.registers.pc += 2;
            cpu.fetched_data = bus_read(cpu, addr) as u16;
            EMULATOR.lock().unwrap().cycles(cpu, 1);
        }
    }
}