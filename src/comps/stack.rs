use super::{cpu::CPUContext, bus::{bus_write, bus_read}};

pub fn stack_push(cpu: &mut CPUContext, data: u8) {
    cpu.registers.sp -= 1;
    bus_write(cpu, cpu.registers.sp, data);
}

pub fn stack_push16(cpu: &mut CPUContext, data: u16) {
    stack_push(cpu, (data >> 8) as u8);
    stack_push(cpu, data as u8);
}

pub fn stack_pop(cpu: &mut CPUContext) -> u8 {
    cpu.registers.sp += 1;
    bus_read(cpu, cpu.registers.sp - 1)
}

pub fn stack_pop16(cpu: &mut CPUContext) -> u16 {
    let lo = stack_pop(cpu) as u16;
    let hi = stack_pop(cpu) as u16;

    (hi << 8) | lo
}