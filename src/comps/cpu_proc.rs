use crate::comps::{
    instructions::{AddrMode, CondType, InstType, RegType},
    stack::*,
};

use super::{
    bus::{bus_read, bus_write, bus_write16},
    cpu::CPUContext,
    emu::EMULATOR,
};

fn proc_none(_cpu: &mut CPUContext) {
    panic!("INVALID INSTRUCTION!\n")
}

fn proc_nop(_cpu: &mut CPUContext) {}

fn proc_ld(cpu: &mut CPUContext) {
    if cpu.dest_is_mem {
        // For instance LD (BC), A
        if cpu.cur_inst.reg2.is_some() && is_16_bit(cpu.cur_inst.reg2.unwrap()) {
            // NOTICE NOTICE NOTICE: reg2 can be None?
            EMULATOR.lock().unwrap().cycles(cpu, 1);
            bus_write16(cpu, cpu.mem_dest, cpu.fetched_data);
        } else {
            bus_write(cpu, cpu.mem_dest, cpu.fetched_data as u8); // NOTICE: He passes fetched_data raw (as a u16). Will this write to multiple places?
        }

        EMULATOR.lock().unwrap().cycles(cpu, 1);

        return;
    }

    if cpu.cur_inst.mode == AddrMode::HLxSPR {
        let h_flag = 0x10 <= (cpu.read_reg(cpu.cur_inst.reg2) & 0xF) + (cpu.fetched_data & 0xF);
        let c_flag = 0x100 <= (cpu.read_reg(cpu.cur_inst.reg2) & 0xFF) + (cpu.fetched_data & 0xFF);

        cpu.set_flags(0, 0, h_flag as u8, c_flag as u8);

        let val = (cpu.read_reg(cpu.cur_inst.reg2) as i32 + cpu.fetched_data as i32) as u16; // NOTICE: He casts to char???
        cpu.set_reg(cpu.cur_inst.reg1, val);

        return;
    }

    cpu.set_reg(cpu.cur_inst.reg1, cpu.fetched_data);
}

fn proc_inc(cpu: &mut CPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_add(1); // NOTICE NOTICE NOTICE: WRAPPING ADD

    if is_16_bit(cpu.cur_inst.reg1.unwrap()) {
        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::HL && cpu.cur_inst.mode == AddrMode::MR {
        let address = cpu.read_reg(Some(RegType::HL));
        val = bus_read(cpu, address) as u16 + 1;
        // NOTICE: He manually extracts bottom byte
        let address = cpu.read_reg(Some(RegType::HL));
        bus_write(cpu, address, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.reg1, val);
        val = cpu.read_reg(cpu.cur_inst.reg1); // NOTICE: Why does this line exist?
    }

    if cpu.cur_opcode & 0x03 == 0x03 {
        // If both bottom bits are set
        // No CPU-flags are changed
        return;
    }

    cpu.set_flags((val == 0) as u8, 0, ((val & 0x0F) == 0) as u8, 2) // NOTICE: This should be double checked
}

fn proc_dec(cpu: &mut CPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(1); // NOTICE NOTICE NOTICE: WRAPPING SUB

    if is_16_bit(cpu.cur_inst.reg1.unwrap()) {
        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::HL && cpu.cur_inst.mode == AddrMode::MR {
        let address = cpu.read_reg(Some(RegType::HL));
        val = bus_read(cpu, address) as u16 - 1;
        // NOTICE: He manually extracts bottom byte
        let address = cpu.read_reg(Some(RegType::HL));
        bus_write(cpu, address, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.reg1, val);
        val = cpu.read_reg(cpu.cur_inst.reg1);
    }

    if cpu.cur_opcode & 0x0B == 0x0B {
        // If both bottom bits are set
        // No CPU-flags are changed
        return;
    }

    cpu.set_flags((val == 0) as u8, 1, ((val & 0x0F) == 0x0F) as u8, 2) // NOTICE: This should be double checked
}

fn proc_rlca(cpu: &mut CPUContext) {
    // NOTICE: I did this somewhat differently
    let mut u = cpu.registers.a;
    let c = (u >> 7) & 1;
    u = (u << 1) | c;
    cpu.registers.a = u;

    cpu.set_flags(0, 0, 0, c);
}

fn proc_add(cpu: &mut CPUContext) {
    let mut val = cpu.read_reg(cpu.cur_inst.reg1) as u32 + cpu.fetched_data as u32;

    let is_16bit = is_16_bit(cpu.cur_inst.reg1.unwrap());

    if is_16bit {
        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::SP {
        val = (cpu.read_reg(cpu.cur_inst.reg1) as i32 + (cpu.fetched_data as i16) as i32) as u32; // NOTICE: Hacky? xD
    }

    let mut z = (val as u8 == 0) as u8;
    let mut h = (0x10 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xF) + (cpu.fetched_data & 0xF)) as u8;
    let mut c =
        (0x100 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFF) + (cpu.fetched_data & 0xFF)) as u8;

    if is_16bit {
        z = 2;
        h = (0x1000 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFFF) + (cpu.fetched_data & 0xFFF))
            as u8;
        let n = cpu.read_reg(cpu.cur_inst.reg1) as u32 + cpu.fetched_data as u32;
        c = (0x10000 <= n) as u8;
    }

    if cpu.cur_inst.reg1.unwrap() == RegType::SP {
        z = 0;
        h = (0x10 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xF) + (cpu.fetched_data & 0xF)) as u8;
        c = (0x100 <= (cpu.read_reg(cpu.cur_inst.reg1) & 0xFF) + (cpu.fetched_data & 0xFF)) as u8;
        // NOTICE: Could be refactored
    }

    cpu.set_reg(cpu.cur_inst.reg1, val as u16);
    cpu.set_flags(z, 0, h, c);
}

fn proc_rrca(cpu: &mut CPUContext) {
    let b = cpu.registers.a & 1;
    cpu.registers.a >>= 1;
    cpu.registers.a |= b << 7;

    cpu.set_flags(0, 0, 0, b);
}

fn proc_stop(_cpu: &mut CPUContext) {
    panic!("STOPPING");
}

fn proc_rla(cpu: &mut CPUContext) {
    let u = cpu.registers.a;
    let c_flag = cpu.flag_c();
    let c = (u >> 7) & 1;

    cpu.registers.a = (u << 1) | c_flag;
    cpu.set_flags(0, 0, 0, c);
}

fn proc_jr(cpu: &mut CPUContext) {
    // NOTICE NOTICE NOTICE: THIS NEEDS VERIFICATION
    let rel = (cpu.fetched_data as u8) as i8;
    let addr = (cpu.registers.pc as i32 + rel as i32) as u16;
    goto_addr(cpu, addr, false);
}

fn proc_rra(cpu: &mut CPUContext) {
    let carry = cpu.flag_c();
    let new_c = cpu.registers.a & 1;

    cpu.registers.a >>= 1;
    cpu.registers.a |= carry << 7;

    cpu.set_flags(0, 0, 0, new_c);
}

fn proc_daa(cpu: &mut CPUContext) {
    let mut u = 0;
    let mut fc = 0;

    if cpu.flag_h() != 0 || (cpu.flag_n() == 0 && 9 < cpu.registers.a & 0x0F) {
        u = 6;
    }

    if cpu.flag_c() != 0 || (cpu.flag_n() == 0 && 0x99 < cpu.registers.a) {
        u |= 0x60;
        fc = 1;
    }

    if cpu.flag_n() != 0 {
        cpu.registers.a = cpu.registers.a.wrapping_sub(u); // NOTICE NOTICE NOTICE: WRAPPING SUB
    } else {
        cpu.registers.a = cpu.registers.a.wrapping_add(u); // NOTICE NOTICE NOTICE: WRAPPING ADD
    }

    let flag_z = (cpu.registers.a == 0) as u8;

    cpu.set_flags(flag_z, 2, 0, fc);
}

fn proc_cpl(cpu: &mut CPUContext) {
    cpu.registers.a = !cpu.registers.a;
    cpu.set_flags(2, 1, 1, 2);
}

fn proc_scf(cpu: &mut CPUContext) {
    cpu.set_flags(2, 0, 0, 1);
}

fn proc_ccf(cpu: &mut CPUContext) {
    let flag_c = cpu.flag_c();
    cpu.set_flags(2, 0, 0, flag_c ^ 1);
}

fn proc_halt(cpu: &mut CPUContext) {
    cpu.halted = true;
}

fn proc_adc(cpu: &mut CPUContext) {
    let u = cpu.fetched_data;
    let a = cpu.registers.a as u16;
    let c = cpu.flag_c() as u16;

    cpu.registers.a = (a + u + c) as u8;

    let flag_z = (cpu.registers.a == 0) as u8;
    cpu.set_flags(
        flag_z,
        0,
        (0xF < (a & 0xF) + (u & 0xF) + c) as u8,
        (0xFF < a + u + c) as u8,
    )
}

fn proc_sub(cpu: &mut CPUContext) {
    let val = cpu
        .read_reg(cpu.cur_inst.reg1)
        .wrapping_sub(cpu.fetched_data); // NOTICE NOTICE NOTICE: WRAPPING SUB

    let z = (val == 0) as u8;
    let h = (((cpu.read_reg(cpu.cur_inst.reg1) & 0xF) as i32 - (cpu.fetched_data & 0xF) as i32) < 0)
        as u8;
    let c = ((cpu.read_reg(cpu.cur_inst.reg1) as i32 - cpu.fetched_data as i32) < 0) as u8; // NOTICE: Control that this works

    cpu.set_reg(cpu.cur_inst.reg1, val);
    cpu.set_flags(z, 1, h, c);
}

fn proc_sbc(cpu: &mut CPUContext) {
    // NOTICE: val is a u8???
    let val = (cpu.fetched_data + cpu.flag_c() as u16) as u8; // NOTICE: Should flag C be able to be negative?

    let z = (cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(val as u16) == 0) as u8; // NOTICE NOTICE NOTICE: WRAPPING SUB
    let h = (((cpu.read_reg(cpu.cur_inst.reg1) & 0xF) as i32
        - (cpu.fetched_data & 0xF) as i32
        - cpu.flag_c() as i32)
        < 0) as u8;
    let c =
        ((cpu.read_reg(cpu.cur_inst.reg1) as i32 - cpu.fetched_data as i32 - cpu.flag_c() as i32)
            < 0) as u8; // NOTICE: Control that this works

    let val = cpu.read_reg(cpu.cur_inst.reg1).wrapping_sub(val as u16); // NOTICE NOTICE NOTICE: WRAPPING SUB
    cpu.set_reg(cpu.cur_inst.reg1, val);
    cpu.set_flags(z, 1, h, c);
}

fn proc_and(cpu: &mut CPUContext) {
    cpu.registers.a &= cpu.fetched_data as u8; // NOTICE: Will fetched_data write to both register A and F?
    let flag_z = (cpu.registers.a == 0) as u8;
    cpu.set_flags(flag_z, 0, 1, 0);
}

fn proc_xor(cpu: &mut CPUContext) {
    cpu.registers.a ^= cpu.fetched_data as u8; // NOTICE: He explicitly says he only wants the bottom byte of the fetched data here, but not in proc_and()?
    let flag_z = (cpu.registers.a == 0) as u8;
    cpu.set_flags(flag_z, 0, 0, 0);
}

fn proc_or(cpu: &mut CPUContext) {
    cpu.registers.a |= cpu.fetched_data as u8; // NOTICE: He explicitly says he only wants the bottom byte of the fetched data here, but not in proc_and()?
    let flag_z = (cpu.registers.a == 0) as u8;
    cpu.set_flags(flag_z, 0, 0, 0);
}

fn proc_cp(cpu: &mut CPUContext) {
    let n = cpu.registers.a as i32 - cpu.fetched_data as i32;
    let flag_h =
        (((cpu.registers.a & 0x0F) as i16) - (((cpu.fetched_data as u8) & 0x0F) as i16) < 0) as u8;
    cpu.set_flags((n == 0) as u8, 1, flag_h, (n < 0) as u8)
}

fn proc_pop(cpu: &mut CPUContext) {
    let lo = stack_pop(cpu) as u16;
    EMULATOR.lock().unwrap().cycles(cpu, 1);

    let hi = stack_pop(cpu) as u16;
    EMULATOR.lock().unwrap().cycles(cpu, 1);

    let result = (hi << 8) | lo;

    cpu.set_reg(cpu.cur_inst.reg1, result);

    if let RegType::AF = cpu.cur_inst.reg1.unwrap() {
        cpu.set_reg(cpu.cur_inst.reg1, result & 0xFFF0);
    }
}

fn proc_jp(cpu: &mut CPUContext) {
    goto_addr(cpu, cpu.fetched_data, false);
}

fn proc_push(cpu: &mut CPUContext) {
    let hi = (cpu.read_reg(cpu.cur_inst.reg1) >> 8) as u8;
    EMULATOR.lock().unwrap().cycles(cpu, 1);
    stack_push(cpu, hi);

    let lo = cpu.read_reg(cpu.cur_inst.reg1) as u8;
    EMULATOR.lock().unwrap().cycles(cpu, 1);
    stack_push(cpu, lo);

    EMULATOR.lock().unwrap().cycles(cpu, 1);
}

fn proc_ret(cpu: &mut CPUContext) {
    // NOTICE: Something fishy might be going on here (Part 07, 10:05)
    if cpu.cur_inst.cond != CondType::NONE {
        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }

    if check_cond(cpu) {
        let lo = stack_pop(cpu) as u16;
        EMULATOR.lock().unwrap().cycles(cpu, 1);

        let hi = stack_pop(cpu) as u16;
        EMULATOR.lock().unwrap().cycles(cpu, 1);

        let new_pc = (hi << 8) | lo;

        cpu.registers.pc = new_pc;

        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }
}

fn proc_cb(cpu: &mut CPUContext) {
    let op = cpu.fetched_data as u8;
    let reg = decode_reg(op & 0b111);
    let bit = (op >> 3) & 0b111;
    let bit_op = (op >> 6) & 0b11;
    let mut reg_val = cpu.read_reg8(reg);

    EMULATOR.lock().unwrap().cycles(cpu, 1);

    if reg == RegType::HL {
        EMULATOR.lock().unwrap().cycles(cpu, 2);
    }

    match bit_op {
        1 => {
            // BIT
            cpu.set_flags(((reg_val & (1 << bit)) == 0) as u8, 0, 1, 2); // NOTICE: Might need checking
        }
        2 => {
            // RST
            reg_val &= !(1 << bit);
            cpu.set_reg8(reg, reg_val);
        }
        3 => {
            // SET
            reg_val |= 1 << bit;
            cpu.set_reg8(reg, reg_val);
        }
        _ => {
            let c_flag = cpu.flag_c();

            match bit {
                0 => {
                    // RLC
                    let mut set_c = false;
                    let mut result = (reg_val << 1) as u8;

                    if reg_val & (1 << 7) != 0 {
                        result |= 1;
                        set_c = true;
                    }

                    cpu.set_reg8(reg, result);
                    cpu.set_flags((result == 0) as u8, 0, 0, set_c as u8);
                }
                1 => {
                    // RRC
                    let old = reg_val;
                    reg_val >>= 1;
                    reg_val |= old << 7;

                    cpu.set_reg8(reg, reg_val);
                    cpu.set_flags((reg_val == 0) as u8, 0, 0, old & 1);
                }
                2 => {
                    // RL
                    let old = reg_val;
                    reg_val <<= 1;
                    reg_val |= c_flag;

                    cpu.set_reg8(reg, reg_val);
                    cpu.set_flags((reg_val == 0) as u8, 0, 0, !!(old & 0x80)); // NOTICE: !! or "(as bool) as u8"?
                }
                3 => {
                    // RR
                    let old = reg_val;
                    reg_val >>= 1;

                    reg_val |= c_flag << 7; // NOTICE: If the C-flag can only have values 0, 1 and 2, how does this make any sense????????

                    cpu.set_reg8(reg, reg_val);
                    cpu.set_flags((reg_val == 0) as u8, 0, 0, old & 1);
                }
                4 => {
                    // SLA
                    let old = reg_val;
                    reg_val <<= 1;

                    cpu.set_reg8(reg, reg_val);
                    cpu.set_flags((reg_val == 0) as u8, 0, 0, !!(old & 0x80)); // NOTICE: !! or "(as bool) as u8"?
                }
                5 => {
                    // SRA
                    let u = reg_val >> 1; // NOTICE: He does "let u = reg_val as i8 >> 1;""

                    cpu.set_reg8(reg, u);
                    cpu.set_flags((u == 0) as u8, 0, 0, reg_val & 1);
                }
                6 => {
                    // SWAP (nibbles)
                    reg_val = ((reg_val & 0x0F) << 4) | (reg_val >> 4);
                    cpu.set_reg8(reg, reg_val);
                    cpu.set_flags((reg_val == 0) as u8, 0, 0, 0);
                }
                7 => {
                    // SRL
                    let u = reg_val >> 1;
                    cpu.set_reg8(reg, u);
                    cpu.set_flags((u == 0) as u8, 0, 0, reg_val & 1);
                }
                _ => panic!("INVALID CB: {op}"),
            }
        }
    }
}

fn proc_call(cpu: &mut CPUContext) {
    goto_addr(cpu, cpu.fetched_data, true);
}

fn proc_reti(cpu: &mut CPUContext) {
    cpu.int_master_enabled = true;
    proc_ret(cpu);
}

fn proc_ldh(cpu: &mut CPUContext) {
    // LDH instructions either have reg1 = Some(RT::A) or reg1 = None
    match cpu.cur_inst.reg1 {
        Some(rt) => {
            let val = bus_read(cpu, cpu.fetched_data | 0xFF00) as u16;
            cpu.set_reg(Some(rt), val);
        }
        None => bus_write(cpu, cpu.mem_dest | 0xFF00, cpu.registers.a),
    }

    EMULATOR.lock().unwrap().cycles(cpu, 1);
}

fn proc_jphl(cpu: &mut CPUContext) {
    panic!("PROCESS NOT YET IMPLEMENTED");
}

fn proc_di(cpu: &mut CPUContext) {
    cpu.int_master_enabled = false;
}

fn proc_ei(cpu: &mut CPUContext) {
    cpu.enabling_ime = true;
}

fn proc_rst(cpu: &mut CPUContext) {
    goto_addr(cpu, cpu.cur_inst.param.unwrap() as u16, true) // NOTICE: Type coersion to u16?
}

fn proc_err(cpu: &mut CPUContext) {
    panic!("PROCESS NOT YET IMPLEMENTED");
}

fn is_16_bit(rt: RegType) -> bool {
    RegType::AF as usize <= rt as usize // NOTICE: Pretty sure this should work?
}

fn check_cond(cpu: &mut CPUContext) -> bool {
    let z = cpu.flag_z() != 0;
    let c = cpu.flag_c() != 0;

    type CT = CondType;
    match cpu.cur_inst.cond {
        CT::NONE => true,
        CT::C => c,
        CT::NC => !c,
        CT::Z => z,
        CT::NZ => !z,
    }
}

fn goto_addr(cpu: &mut CPUContext, address: u16, push_pc: bool) {
    if check_cond(cpu) {
        if push_pc {
            EMULATOR.lock().unwrap().cycles(cpu, 2);
            stack_push16(cpu, cpu.registers.pc);
        }

        cpu.registers.pc = address;
        EMULATOR.lock().unwrap().cycles(cpu, 1);
    }
}

fn proc_dummy(cpu: &mut CPUContext) {
    panic!(
        "PROCESS NOT IMPLEMENTED FOR INSTRUCTION: {}",
        cpu.cur_inst.inst_type
    )
}

pub const PROCESSORS: [&dyn Fn(&mut CPUContext) -> (); 48] = [
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
    &proc_err,
    //CB instructions...
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
    &proc_dummy,
];

pub fn proc_by_inst(inst_type: InstType) -> &'static dyn Fn(&mut CPUContext) -> () {
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
