use crate::comps::{cpu::CPUContext, stack::stack_push16};

#[derive(Clone, Copy)]
pub enum InterruptType {
    VBlank = 1,
    LCDStat = 2,
    Timer = 4,
    Serial = 8,
    Joypad = 16
}

fn int_check(cpu: &mut CPUContext, address: u16, it: InterruptType) -> bool {
    if cpu.int_flags & it as u8 != 0 && cpu.get_ie_reg() & it as u8 != 0 {
        int_handle(cpu, address);
        cpu.int_flags &= !(it as u8);
        cpu.halted = false;
        cpu.int_master_enabled = false;

        return true;
    }

    false
}

pub fn int_handle(cpu: &mut CPUContext, address: u16) {
    stack_push16(cpu, cpu.registers.pc);
    cpu.registers.pc = address;
}

pub fn handle_interrupts(cpu: &mut CPUContext) {
    type IT = InterruptType;
    if int_check(cpu, 0x40, IT::VBlank) {

    } else if int_check(cpu, 0x48, IT::LCDStat) {

    } else if int_check(cpu, 0x50, IT::Timer) {
        
    } else if int_check(cpu, 0x58, IT::Serial) {
        
    } else if int_check(cpu, 0x60, IT::Joypad) {
        
    }
}
