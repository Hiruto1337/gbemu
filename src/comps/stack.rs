use super::{cpu::CPUContext, bus::{bus_write, bus_read}, ppu::PPUContext};

pub fn stack_push(cpu: &mut CPUContext, ppu: &mut PPUContext, data: u8) {
    cpu.registers.sp -= 1;
    bus_write(cpu, ppu, cpu.registers.sp, data);
}

pub fn stack_push16(cpu: &mut CPUContext, ppu: &mut PPUContext, data: u16) {
    stack_push(cpu, ppu, (data >> 8) as u8);
    stack_push(cpu, ppu, data as u8);
}

pub fn stack_pop(cpu: &mut CPUContext, ppu: &mut PPUContext) -> u8 {
    cpu.registers.sp += 1;
    bus_read(cpu, ppu, cpu.registers.sp - 1)
}

pub fn stack_pop16(cpu: &mut CPUContext, ppu: &mut PPUContext) -> u16 {
    let lo = stack_pop(cpu, ppu) as u16;
    let hi = stack_pop(cpu, ppu) as u16;

    (hi << 8) | lo
}