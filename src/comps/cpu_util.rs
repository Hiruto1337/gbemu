use crate::comps::{cpu::CPUContext, instructions::RegType, bus::{bus_read, bus_write}};

pub fn read_reg8(cpu: &mut CPUContext, rt: RegType) -> u8 {
    type RT = RegType;
    match rt {
        RT::A => cpu.registers.a,
        RT::F => cpu.registers.f,
        RT::B => cpu.registers.b,
        RT::C => cpu.registers.c,
        RT::D => cpu.registers.d,
        RT::E => cpu.registers.e,
        RT::H => cpu.registers.h,
        RT::L => cpu.registers.l,
        RT::HL => bus_read(cpu, read_reg(cpu, Some(RT::HL))),
        _ => panic!("INVALID REG8: {rt:?}")
    }
}

pub fn read_reg(cpu: &CPUContext, rt: Option<RegType>) -> u16 {
    type RT = RegType;
    // println!("{:#X}", cpu.cur_opcode);
    match rt.unwrap() {
        RT::NONE => panic!("UNKNOWN REGISTER TYPE"),
        RT::A => return cpu.registers.a as u16,
        RT::F => return cpu.registers.f as u16,
        RT::B => return cpu.registers.b as u16,
        RT::C => return cpu.registers.c as u16,
        RT::D => return cpu.registers.d as u16,
        RT::E => return cpu.registers.e as u16,
        RT::H => return cpu.registers.h as u16,
        RT::L => return cpu.registers.l as u16,

        // NOTICE NOTICE NOTICE
        RT::AF => return ((cpu.registers.a as u16) << 8) | (cpu.registers.f as u16),
        RT::BC => return ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16),
        RT::DE => return ((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16),
        RT::HL => return ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16),

        RT::PC => return cpu.registers.pc,
        RT::SP => return cpu.registers.sp
    }
}

pub fn set_reg8(cpu: &mut CPUContext, rt: RegType, val: u8) {
    type RT = RegType;
    match rt {
        RT::A => cpu.registers.a = val,
        RT::F => cpu.registers.f = val,
        RT::B => cpu.registers.b = val,
        RT::C => cpu.registers.c = val,
        RT::D => cpu.registers.d = val,
        RT::E => cpu.registers.e = val,
        RT::H => cpu.registers.h = val,
        RT::L => cpu.registers.l = val,
        RT::HL => bus_write(cpu, read_reg(cpu, Some(RT::HL)), val),
        _ => panic!("INVALID REG8: {rt:?}")
    }
}

pub fn set_reg(cpu: &mut CPUContext, rt: Option<RegType>, val: u16) {
    let rt = rt.unwrap();

    type RT = RegType;
    match rt {
        RT::NONE => panic!("UNKNOWN REGISTER TYPE"),
        RT::A => cpu.registers.a = val as u8,
        RT::F => cpu.registers.f = val as u8,
        RT::B => cpu.registers.b = val as u8,
        RT::C => cpu.registers.c = val as u8,
        RT::D => cpu.registers.d = val as u8,
        RT::E => cpu.registers.e = val as u8,
        RT::H => cpu.registers.h = val as u8,
        RT::L => cpu.registers.l = val as u8,

        // NOTICE: Is this even implemented correctly?
        RT::AF => {
            cpu.registers.a = (val >> 8) as u8;
            cpu.registers.f = val as u8;
        },
        RT::BC => {
            cpu.registers.b = (val >> 8) as u8;
            cpu.registers.c = val as u8;
        },
        RT::DE => {
            cpu.registers.d = (val >> 8) as u8;
            cpu.registers.e = val as u8;
        },
        RT::HL => {
            cpu.registers.h = (val >> 8) as u8;
            cpu.registers.l = val as u8;
        },

        RT::PC => cpu.registers.pc = val,
        RT::SP => cpu.registers.sp = val
    }
}

pub fn get_int_flags(cpu: &CPUContext) -> u8 {
    cpu.int_flags
}

pub fn set_int_flags(cpu: &mut CPUContext, value: u8) {
    cpu.int_flags = value;
}