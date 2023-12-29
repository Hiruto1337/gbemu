use crate::comps::{cpu::CPUContext, instructions::RegType, bus::{bus_read, bus_write}};

use super::ppu::PPUContext;

impl CPUContext {
    pub fn read_reg8(&mut self, ppu: &PPUContext, rt: RegType) -> u8 {
        type RT = RegType;
        match rt {
            RT::A => self.registers.a,
            RT::F => self.registers.f,
            RT::B => self.registers.b,
            RT::C => self.registers.c,
            RT::D => self.registers.d,
            RT::E => self.registers.e,
            RT::H => self.registers.h,
            RT::L => self.registers.l,
            RT::HL => {
                let address = self.read_reg(Some(RT::HL));
                bus_read(self, ppu, address)
            },
            _ => panic!("INVALID REG8: {rt:?}")
        }
    }
    
    pub fn read_reg(&mut self, rt: Option<RegType>) -> u16 {
        type RT = RegType;
        match rt.unwrap() {
            RT::NONE => panic!("UNKNOWN REGISTER TYPE"),
            RT::A => return self.registers.a as u16,
            RT::F => return self.registers.f as u16,
            RT::B => return self.registers.b as u16,
            RT::C => return self.registers.c as u16,
            RT::D => return self.registers.d as u16,
            RT::E => return self.registers.e as u16,
            RT::H => return self.registers.h as u16,
            RT::L => return self.registers.l as u16,
    
            RT::AF => return ((self.registers.a as u16) << 8) | self.registers.f as u16,
            RT::BC => return ((self.registers.b as u16) << 8) | self.registers.c as u16,
            RT::DE => return ((self.registers.d as u16) << 8) | self.registers.e as u16,
            RT::HL => return ((self.registers.h as u16) << 8) | self.registers.l as u16,
    
            RT::PC => return self.registers.pc,
            RT::SP => return self.registers.sp
        }
    }
    
    pub fn set_reg8(&mut self, ppu: &mut PPUContext, rt: RegType, val: u8) {
        type RT = RegType;
        match rt {
            RT::A => self.registers.a = val,
            RT::F => self.registers.f = val,
            RT::B => self.registers.b = val,
            RT::C => self.registers.c = val,
            RT::D => self.registers.d = val,
            RT::E => self.registers.e = val,
            RT::H => self.registers.h = val,
            RT::L => self.registers.l = val,
            RT::HL => {
                let address = self.read_reg(Some(RT::HL));
                bus_write(self, ppu, address, val);
            },
            _ => panic!("INVALID REG8: {rt:?}")
        }
    }
    
    pub fn set_reg(&mut self, rt: Option<RegType>, val: u16) {    
        type RT = RegType;
        match rt.unwrap() {
            RT::NONE => panic!("UNKNOWN REGISTER TYPE"),
            RT::A => self.registers.a = val as u8,
            RT::F => self.registers.f = val as u8,
            RT::B => self.registers.b = val as u8,
            RT::C => self.registers.c = val as u8,
            RT::D => self.registers.d = val as u8,
            RT::E => self.registers.e = val as u8,
            RT::H => self.registers.h = val as u8,
            RT::L => self.registers.l = val as u8,
    
            RT::AF => {
                self.registers.a = (val >> 8) as u8;
                self.registers.f = val as u8;
            },
            RT::BC => {
                self.registers.b = (val >> 8) as u8;
                self.registers.c = val as u8;
            },
            RT::DE => {
                self.registers.d = (val >> 8) as u8;
                self.registers.e = val as u8;
            },
            RT::HL => {
                self.registers.h = (val >> 8) as u8;
                self.registers.l = val as u8;
            },
            RT::PC => self.registers.pc = val,
            RT::SP => self.registers.sp = val
        }
    }
    
    pub fn get_int_flags(&self) -> u8 {
        self.int_flags
    }
    
    pub fn set_int_flags(&mut self, value: u8) {
        self.int_flags = value;
    }
}
