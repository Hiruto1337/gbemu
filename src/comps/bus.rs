use super::{cart::CART, ram::RAM, io::{io_read, io_write}, cpu::CPUContext};

pub fn bus_read(cpu: &mut CPUContext, address: u16) -> u8 {
    let cart = &mut CART.lock().unwrap();
    match address {
        addr if addr < 0x8000 => cart.read(address), // ROM data
        addr if addr < 0xA000 => {println!("UNSUPPORTED: Bus.read({address:04X}): CHR RAM, BG Map 1, BG Map 2"); return 0;}, // Char/map data
        addr if addr < 0xC000 => cart.read(address), // Cartridge RAM
        addr if addr < 0xE000 => RAM.lock().unwrap().wram_read(address), // WRAM (Working RAM)
        addr if addr < 0xFE00 => 0, // Reserved echo RAM
        addr if addr < 0xFEA0 => {println!("UNSUPPORTED: Bus.read({address:04X}): Object Attribute Memory"); return 0;}, // OAM
        addr if addr < 0xFF00 => 0, // Unusable reserved,
        addr if addr < 0xFF80 => io_read(cpu, address), // I/O Registers
        addr if addr == 0xFFFF => cpu.get_ie_reg(), // CPU enable register
        _ => RAM.lock().unwrap().hram_read(address) // HRAM (High RAM)
    }
}

pub fn bus_write(cpu: &mut CPUContext, address: u16, value: u8) {
    let cart = &mut CART.lock().unwrap();
    match address {
        addr if addr < 0x8000 => cart.write(address, value), // ROM data
        addr if addr < 0xA000 => println!("UNSUPPORTED: Bus.write({address:04X}): CHR RAM, BG Map 1, BG Map 2"), // Char/map data
        addr if addr < 0xC000 => cart.write(address, value), // Cartridge RAM
        addr if addr < 0xE000 => RAM.lock().unwrap().wram_write(address, value), // WRAM (Working RAM)
        addr if addr < 0xFE00 => println!("UNSUPPORTED: Bus.write({address:04X}): Reserved echo RAM"), // Reserved echo RAM
        addr if addr < 0xFEA0 => println!("UNSUPPORTED: Bus.write({address:04X}): Object Attribute Memory"), // OAM
        addr if addr < 0xFF00 => println!("UNSUPPORTED: Bus.write({address:04X}): Unusable reserved"), // Unusable reserved,
        addr if addr < 0xFF80 => io_write(cpu, address, value), // I/O Registers
        addr if addr == 0xFFFF => cpu.set_ie_reg(value), // CPU enable register
        _ => RAM.lock().unwrap().hram_write(address, value) // HRAM (High RAM)
    }
}

pub fn bus_read16(cpu: &mut CPUContext, address: u16) -> u16 {
    let lo = bus_read(cpu, address) as u16;
    let hi = bus_read(cpu, address + 1) as u16;

    return (hi << 8) | lo;
}

pub fn bus_write16(cpu: &mut CPUContext, address: u16, value: u16) {
    bus_write(cpu, address + 1, (value >> 8) as u8);
    bus_write(cpu, address, value as u8);
}

// 0x0000 - 0x3FFF : ROM Bank 0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
// 0x8000 - 0x97FF : CHR RAM
// 0x9800 - 0x9BFF : BG Map 1
// 0x9C00 - 0x9FFF : BG Map 2
// 0xA000 - 0xBFFF : Cartridge RAM
// 0xC000 - 0xCFFF : RAM Bank 0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
// 0xE000 - 0xFDFF : Reserved - Echo RAM
// 0xFE00 - 0xFE9F : Object Attribute Memory
// 0xFEA0 - 0xFEFF : Reserved - Unusable
// 0xFF00 - 0xFF7F : I/O Registers
// 0xFF80 - 0xFFFE : Zero Page
