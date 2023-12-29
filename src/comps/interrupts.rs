use crate::comps::{cpu::CPUContext, stack::stack_push16};

use super::ppu::PPUContext;

#[derive(Clone, Copy)]
pub enum InterruptType {
    VBlank  = 0b00001,
    LCDStat = 0b00010,
    Timer   = 0b00100,
    Serial  = 0b01000,
    Joypad  = 0b10000
}

fn int_check(cpu: &mut CPUContext, ppu: &mut PPUContext, address: u16, it: InterruptType) -> bool {
    if cpu.int_flags & it as u8 != 0 && cpu.get_ie_reg() & it as u8 != 0 {
        int_handle(cpu, ppu, address);
        cpu.int_flags &= !(it as u8);
        cpu.halted = false;
        cpu.int_master_enabled = false;

        return true;
    }

    false
}

pub fn int_handle(cpu: &mut CPUContext, ppu: &mut PPUContext, address: u16) {
    stack_push16(cpu, ppu, cpu.registers.pc);
    cpu.registers.pc = address;
}

pub fn handle_interrupts(cpu: &mut CPUContext, ppu: &mut PPUContext) {
    type IT = InterruptType;
    if int_check(cpu, ppu, 0x40, IT::VBlank) {

    } else if int_check(cpu, ppu, 0x48, IT::LCDStat) {

    } else if int_check(cpu, ppu, 0x50, IT::Timer) {
        
    } else if int_check(cpu, ppu, 0x58, IT::Serial) {
        
    } else if int_check(cpu, ppu, 0x60, IT::Joypad) {
        
    }
}
