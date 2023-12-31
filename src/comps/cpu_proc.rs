use crate::comps::{
    instructions::{AddrMode, CondType, InstType, RegType},
    stack::*,
};

use super::{
    bus::{bus_read, bus_write, bus_write16},
    cpu::CPUContext,
    emu::EMULATOR, ppu::PPUContext,
};

fn proc_none(_cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    panic!("INVALID INSTRUCTION!\n")
}

fn proc_nop(_cpu: &mut CPUContext, _ppu: &mut PPUContext) {}

fn proc_ld(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    if cpu.dest_is_mem {
        if cpu.cur_inst.reg2.is_some() && is_16_bit(cpu.cur_inst.reg2.unwrap()) {
            EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
            bus_write16(cpu, ppu, cpu.mem_dest, cpu.fetched_data);
        } else {
            bus_write(cpu, ppu, cpu.mem_dest, cpu.fetched_data as u8);
        }

        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

        return;
    }

    if cpu.cur_inst.mode == AddrMode::HLxSPR {
        let h_flag = 0x10 <= (cpu.read_reg(cpu.cur_inst.reg2) & 0xF) + (cpu.fetched_data & 0xF);
        let c_flag = 0x100 <= (cpu.read_reg(cpu.cur_inst.reg2) & 0xFF) + (cpu.fetched_data & 0xFF);

        cpu.set_flags(Some(false), Some(false), Some(h_flag), Some(c_flag));

        let val = cpu.read_reg(cpu.cur_inst.reg2) as i32 + (cpu.fetched_data as i8) as i32;
        cpu.set_reg(cpu.cur_inst.reg1, val as u16);

        return;
    }

    cpu.set_reg(cpu.cur_inst.reg1, cpu.fetched_data);
}

fn proc_inc(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_add(1);

    if is_16_bit(cpu.cur_inst.reg1.unwrap()) {
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::HL && cpu.cur_inst.mode == AddrMode::MR {
        let address = cpu.read_reg(Some(RegType::HL));
        val = (bus_read(cpu, ppu, address) as u16 + 1) & 0xFF;
        bus_write(cpu, ppu, address, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.reg1, val);
        val = cpu.read_reg(cpu.cur_inst.reg1);
    }

    if cpu.cur_opcode & 0x3 == 0x3 {
        return;
    }

    cpu.set_flags(Some(val == 0), Some(false), Some(val & 0xF == 0), None);
}

fn proc_dec(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(1);

    if is_16_bit(cpu.cur_inst.reg1.unwrap()) {
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::HL && cpu.cur_inst.mode == AddrMode::MR {
        let address = cpu.read_reg(Some(RegType::HL));
        val = (bus_read(cpu, ppu, address) as u16).wrapping_sub(1);
        bus_write(cpu, ppu, address, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.reg1, val);
        val = cpu.read_reg(cpu.cur_inst.reg1);
    }

    if cpu.cur_opcode & 0xB == 0xB {
        return;
    }

    cpu.set_flags(Some(val == 0), Some(true), Some(val & 0xF == 0xF), None)
}

fn proc_rlca(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let mut u = cpu.registers.a;
    let c = (u >> 7) & 1;
    u = (u << 1) | c;
    cpu.registers.a = u;

    cpu.set_flags(Some(false), Some(false), Some(false), Some(c != 0));
}

fn proc_add(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1) as u32 + cpu.fetched_data as u32;

    let is_16bit = is_16_bit(cpu.cur_inst.reg1.unwrap());

    if is_16bit {
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::SP {
        val = (cpu.read_reg(cpu.cur_inst.reg1) as i32 + (cpu.fetched_data as i8) as i32) as u32;
    }

    let mut z = Some(val as u8 == 0);
    let mut h = Some(0x10 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xF) + (cpu.fetched_data & 0xF));
    let mut c = Some(0x100 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFF) + (cpu.fetched_data & 0xFF));

    if is_16bit {
        z = None;
        h = Some(0x1000 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFFF) + (cpu.fetched_data & 0xFFF));
        let n = cpu.read_reg(cpu.cur_inst.reg1) as u32 + cpu.fetched_data as u32;
        c = Some(0x10000 <= n);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::SP {
        z = Some(false);
        h = Some(0x10 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xF) + (cpu.fetched_data & 0xF));
        c = Some(0x100 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFF) + (cpu.fetched_data & 0xFF));
    }

    cpu.set_reg(cpu.cur_inst.reg1, val as u16);
    cpu.set_flags(z, Some(false), h, c);
}

fn proc_rrca(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let b = cpu.registers.a & 1;
    cpu.registers.a >>= 1;
    cpu.registers.a |= b << 7;

    cpu.set_flags(Some(false), Some(false), Some(false), Some(b != 0));
}

fn proc_stop(_cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    panic!("STOP");
}

fn proc_rla(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let u = cpu.registers.a;
    let c_flag = cpu.flag_c();
    let c = (u >> 7) & 1;

    cpu.registers.a = (u << 1) | c_flag as u8;
    cpu.set_flags(Some(false), Some(false), Some(false), Some(c != 0));
}

fn proc_jr(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let rel = cpu.fetched_data as i8;
    let addr = (cpu.registers.pc as i32 + rel as i32) as u16;
    goto_addr(cpu, ppu, addr, false);
}

fn proc_rra(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let carry = cpu.flag_c() as u8;
    let new_c = cpu.registers.a & 1;

    cpu.registers.a >>= 1;
    cpu.registers.a |= carry << 7;

    cpu.set_flags(Some(false), Some(false), Some(false), Some(new_c != 0));
}

fn proc_daa(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let mut u = 0;
    let mut fc = 0;

    if cpu.flag_h() || (!cpu.flag_n() && 9 < cpu.registers.a & 0xF) {
        u = 6;
    }

    if cpu.flag_c() || (!cpu.flag_n() && 0x99 < cpu.registers.a) {
        u |= 0x60;
        fc = 1;
    }

    if cpu.flag_n() {
        cpu.registers.a = cpu.registers.a.wrapping_sub(u);
    } else {
        cpu.registers.a = cpu.registers.a.wrapping_add(u);
    }

    let flag_z = cpu.registers.a == 0;

    cpu.set_flags(Some(flag_z), None, Some(false), Some(fc != 0));
}

fn proc_cpl(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.registers.a = !cpu.registers.a;
    cpu.set_flags(None, Some(true), Some(true), None);
}

fn proc_scf(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.set_flags(None, Some(false), Some(false), Some(true));
}

fn proc_ccf(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let flag_c = cpu.flag_c() as u8;
    cpu.set_flags(None, Some(false), Some(false), Some(flag_c ^ 1 != 0));
}

fn proc_halt(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.halted = true;
}

fn proc_adc(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let u = cpu.fetched_data;
    let a = cpu.registers.a as u16;
    let c = cpu.flag_c() as u16;

    cpu.registers.a = (a + u + c) as u8;

    let flag_z = cpu.registers.a == 0;
    let flag_h = 0xF < (a & 0xF) + (u & 0xF) + c;
    let flag_c = 0xFF < a + u + c;
    cpu.set_flags(Some(flag_z), Some(false), Some(flag_h), Some(flag_c))
}

fn proc_sub(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let val = cpu
        .read_reg(cpu.cur_inst.reg1)
        .wrapping_sub(cpu.fetched_data);

    let z = val == 0;
    let h = (cpu.read_reg(cpu.cur_inst.reg1) & 0xF).checked_sub(cpu.fetched_data & 0xF).is_none();
    let c = cpu.read_reg(cpu.cur_inst.reg1).checked_sub(cpu.fetched_data).is_none();

    cpu.set_reg(cpu.cur_inst.reg1, val);
    cpu.set_flags(Some(z), Some(true), Some(h), Some(c));
}

fn proc_sbc(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let val = (cpu.fetched_data + cpu.flag_c() as u16) as u8;

    let z = cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(val as u16) == 0;
    let h = (cpu.read_reg(cpu.cur_inst.reg1) & 0xF)
        .checked_sub((cpu.fetched_data & 0xF) + cpu.flag_c() as u16)
        .is_none();
    let c = cpu
        .read_reg(cpu.cur_inst.reg1)
        .checked_sub(cpu.fetched_data + cpu.flag_c() as u16)
        .is_none();

    let val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(val as u16);
    cpu.set_reg(cpu.cur_inst.reg1, val);
    cpu.set_flags(Some(z), Some(true), Some(h), Some(c));
}

fn proc_and(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.registers.a &= cpu.fetched_data as u8;
    let flag_z = cpu.registers.a == 0;
    cpu.set_flags(Some(flag_z), Some(false), Some(true), Some(false));
}

fn proc_xor(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.registers.a ^= cpu.fetched_data as u8;
    let flag_z = cpu.registers.a == 0;
    cpu.set_flags(Some(flag_z), Some(false), Some(false), Some(false));
}

fn proc_or(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.registers.a |= cpu.fetched_data as u8;
    let flag_z = cpu.registers.a == 0;
    cpu.set_flags(Some(flag_z), Some(false), Some(false), Some(false));
}

fn proc_cp(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    let n = cpu.registers.a as i32 - cpu.fetched_data as i32;
    let flag_h = (cpu.registers.a & 0xF)
        .checked_sub((cpu.fetched_data & 0xF) as u8)
        .is_none();
    cpu.set_flags(Some(n == 0), Some(true), Some(flag_h), Some(n < 0))
}

fn proc_pop(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let lo = stack_pop(cpu, ppu) as u16;
    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

    let hi = stack_pop(cpu, ppu) as u16;
    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

    let result = (hi << 8) | lo;

    cpu.set_reg(cpu.cur_inst.reg1, result);

    if cpu.cur_inst.reg1.unwrap() == RegType::AF {
        cpu.set_reg(cpu.cur_inst.reg1, result & 0xFFF0);
    }
}

fn proc_jp(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    goto_addr(cpu, ppu, cpu.fetched_data, false);
}

fn proc_push(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let reg_1 = cpu.read_reg(cpu.cur_inst.reg1);
    let hi = (reg_1 >> 8) as u8;
    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    stack_push(cpu, ppu, hi);

    let lo = reg_1 as u8;
    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    stack_push(cpu, ppu, lo);

    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
}

fn proc_ret(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    if cpu.cur_inst.cond != CondType::NONE {
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }

    if check_cond(cpu, ppu) {
        let lo = stack_pop(cpu, ppu) as u16;
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

        let hi = stack_pop(cpu, ppu) as u16;
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

        let new_pc = (hi << 8) | lo;

        cpu.registers.pc = new_pc;

        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }
}

fn proc_cb(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    let op = cpu.fetched_data as u8;
    let reg = decode_reg(op & 0b111);
    let bit = (op >> 3) & 0b111;
    let bit_op = (op >> 6) & 0b11;
    let mut reg_val = cpu.read_reg8(ppu, reg);

    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);

    if reg == RegType::HL {
        EMULATOR.write().unwrap().cycles(cpu, ppu, 2);
    }

    match bit_op {
        1 => {
            // BIT
            let z = reg_val & (1 << bit) == 0;
            cpu.set_flags(Some(z), Some(false), Some(true), None);
        }
        2 => {
            // RST
            reg_val &= !(1 << bit);
            cpu.set_reg8(ppu, reg, reg_val);
        }
        3 => {
            // SET
            reg_val |= 1 << bit;
            cpu.set_reg8(ppu, reg, reg_val);
        }
        _ => {
            let c_flag = cpu.flag_c() as u8;

            match bit {
                0 => {
                    // RLC
                    let mut set_c = false;
                    let mut result = (reg_val << 1) as u8;

                    if reg_val & (1 << 7) != 0 {
                        result |= 1;
                        set_c = true;
                    }

                    cpu.set_reg8(ppu, reg, result);
                    cpu.set_flags(Some(result == 0), Some(false), Some(false), Some(set_c));
                }
                1 => {
                    // RRC
                    let old = reg_val;
                    reg_val >>= 1;
                    reg_val |= old << 7;

                    cpu.set_reg8(ppu, reg, reg_val);
                    cpu.set_flags(
                        Some(reg_val == 0),
                        Some(false),
                        Some(false),
                        Some(old & 1 != 0),
                    );
                }
                2 => {
                    // RL
                    let old = reg_val;
                    reg_val <<= 1;
                    reg_val |= c_flag;

                    cpu.set_reg8(ppu, reg, reg_val);
                    cpu.set_flags(
                        Some(reg_val == 0),
                        Some(false),
                        Some(false),
                        Some(old & 0x80 != 0),
                    );
                }
                3 => {
                    // RR
                    let old = reg_val;
                    reg_val >>= 1;

                    reg_val |= c_flag << 7;

                    cpu.set_reg8(ppu, reg, reg_val);
                    cpu.set_flags(
                        Some(reg_val == 0),
                        Some(false),
                        Some(false),
                        Some(old & 1 != 0),
                    );
                }
                4 => {
                    // SLA
                    let old = reg_val;
                    reg_val <<= 1;

                    cpu.set_reg8(ppu, reg, reg_val);
                    cpu.set_flags(
                        Some(reg_val == 0),
                        Some(false),
                        Some(false),
                        Some(old & 0x80 != 0),
                    );
                }
                5 => {
                    // SRA
                    let u = ((reg_val as i8) >> 1) as u8;

                    cpu.set_reg8(ppu, reg, u);
                    cpu.set_flags(
                        Some(u == 0),
                        Some(false),
                        Some(false),
                        Some(reg_val & 1 != 0),
                    );
                }
                6 => {
                    // SWAP (nibbles)
                    reg_val = ((reg_val & 0xF) << 4) | ((reg_val & 0xF0) >> 4);
                    cpu.set_reg8(ppu, reg, reg_val);
                    cpu.set_flags(Some(reg_val == 0), Some(false), Some(false), Some(false));
                }
                7 => {
                    // SRL
                    let u = reg_val >> 1;
                    cpu.set_reg8(ppu, reg, u);
                    cpu.set_flags(
                        Some(u == 0),
                        Some(false),
                        Some(false),
                        Some(reg_val & 1 != 0),
                    );
                }
                _ => panic!("INVALID CB: {op}"),
            }
        }
    }
}

fn proc_call(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    goto_addr(cpu, ppu, cpu.fetched_data, true);
}

fn proc_reti(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    cpu.int_master_enabled = true;
    proc_ret(cpu, ppu);
}

fn proc_ldh(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    // LDH instructions either have reg1 = Some(RT::A) or reg1 = None
    match cpu.cur_inst.reg1 {
        Some(rt) => {
            let val = bus_read(cpu, ppu, cpu.fetched_data | 0xFF00) as u16;
            cpu.set_reg(Some(rt), val);
        }
        None => bus_write(cpu, ppu, cpu.mem_dest | 0xFF00, cpu.registers.a),
    }

    EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
}

fn proc_jphl(_cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    panic!("PROCESS NOT YET IMPLEMENTED");
}

fn proc_di(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.int_master_enabled = false;
}

fn proc_ei(cpu: &mut CPUContext, _ppu: &mut PPUContext) {
    cpu.enabling_ime = true;
}

fn proc_rst(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    goto_addr(cpu, ppu, cpu.cur_inst.param.unwrap() as u16, true);
}

fn is_16_bit(rt: RegType) -> bool {
    RegType::AF as usize <= rt as usize
}

fn check_cond(cpu: &mut CPUContext, _ppu: &mut PPUContext) -> bool {
    type CT = CondType;
    match cpu.cur_inst.cond {
        CT::NONE => true,
        CT::C => cpu.flag_c(),
        CT::NC => !cpu.flag_c(),
        CT::Z => cpu.flag_z(),
        CT::NZ => !cpu.flag_z(),
    }
}

fn goto_addr(cpu: &mut CPUContext, ppu: &mut PPUContext, address: u16, push_pc: bool) {
    if check_cond(cpu, ppu) {
        if push_pc {
            EMULATOR.write().unwrap().cycles(cpu, ppu, 2);
            stack_push16(cpu, ppu, cpu.registers.pc);
        }

        cpu.registers.pc = address;
        EMULATOR.write().unwrap().cycles(cpu, ppu, 1);
    }
}

pub const PROCESSORS: [&dyn Fn(&mut CPUContext, &mut PPUContext) -> (); 36] = [
    &proc_none,
    &proc_nop,
    &proc_ld,
    &proc_inc,
    &proc_dec,
    &proc_rlca,
    &proc_add,
    &proc_rrca,
    &proc_stop,
    &proc_rla,
    &proc_jr,
    &proc_rra,
    &proc_daa,
    &proc_cpl,
    &proc_scf,
    &proc_ccf,
    &proc_halt,
    &proc_adc,
    &proc_sub,
    &proc_sbc,
    &proc_and,
    &proc_xor,
    &proc_or,
    &proc_cp,
    &proc_pop,
    &proc_jp,
    &proc_push,
    &proc_ret,
    &proc_cb,
    &proc_call,
    &proc_reti,
    &proc_ldh,
    &proc_jphl,
    &proc_di,
    &proc_ei,
    &proc_rst,
];

pub fn proc_by_inst(inst_type: InstType) -> &'static dyn Fn(&mut CPUContext, &mut PPUContext) -> () {
    PROCESSORS[inst_type as usize]
}

pub const RT_LOOKUP: [RegType; 8] = [
    RegType::B,
    RegType::C,
    RegType::D,
    RegType::E,
    RegType::H,
    RegType::L,
    RegType::HL,
    RegType::A,
];

pub fn decode_reg(reg: u8) -> RegType {
    if 0b111 < reg {
        return RegType::NONE;
    }

    RT_LOOKUP[reg as usize]
}
