use std::sync::RwLock;

use super::{cart::CART, lcd::{LCD, LCDMode}};

pub const LINES_PER_FRAME: u8 = 154;
pub const TICKS_PER_LINE: u32 = 456;
pub const Y_RES: u8 = 144;
pub const X_RES: u8 = 160;

#[derive(Clone, Copy)]
pub struct OAMEntry {
    pub y: u8,
    pub x: u8,
    pub tile: u8,
    pub flag: u8
    // pub f_cgb_pn: u8,           // Bit 0, 1 and 2 in byte 3
    // pub f_cgb_vram_bank: bool,  // Bit 3 in byte 3
    // pub f_pn: bool,             // Bit 4 in byte 3
    // pub f_x_flip: bool,         // Bit 5 in byte 3
    // pub f_y_flip: bool,         // Bit 6 in byte 3
    // pub f_bgp: bool             // bit 7 in byte 3
}

pub struct PPUContext {
    pub oam_ram: [OAMEntry; 40],
    pub vram: [u8; 0x2000],
    pub current_frame: u32,
    pub line_ticks: u32,
    pub frame_buffer: [u32; Y_RES as usize * X_RES as usize * 4] // NOTICE: sizeof(32)???
}

pub static PPU: RwLock<PPUContext> = RwLock::new(PPUContext {
    oam_ram: [OAMEntry {y: 0, x: 0, tile: 0, flag: 0} ; 40],
    vram: [0; 0x2000],
    current_frame: 0,
    line_ticks: 0,
    frame_buffer: [0; Y_RES as usize * X_RES as usize * 4]
});

impl PPUContext {
    pub fn init(&mut self) {

        // NOTICE: NEEDS VALIDATION
        // let cart = CART.read().unwrap();
        // let oam_start = 0xFE00;

        // Load OAM into PPU
        // for entry in 0..40 {
        //     let index = oam_start + 4 * entry;
        //     self.oam_ram[entry] = OAMEntry {
        //         y: cart.rom_data[index],
        //         x: cart.rom_data[index + 1],
        //         tile: cart.rom_data[index + 2],
        //         flag: cart.rom_data[index + 3]
        //     }
        // }

        // Load VRAM into PPU
        // self.vram = cart.rom_data[0x8000..0xA000].try_into().unwrap();
    }

    pub fn oam_write(&mut self, mut address: u16, value: u8) {
        // NOTICE: Really needs validation xD
        // oam_read and oam_write might be accessed from the DMA, which won't be using 0xFE00-offset
        if 0xFE00 <= address {
            address -= 0xFE00;
        }

        let entry = address % 4;

        let oam_index = address / 4;

        match entry {
            0 => self.oam_ram[oam_index as usize].y = value,
            1 => self.oam_ram[oam_index as usize].x = value,
            2 => self.oam_ram[oam_index as usize].tile = value,
            3 => self.oam_ram[oam_index as usize].flag = value,
            _ => unreachable!()
        }
    }

    pub fn oam_read(&self, mut address: u16) -> u8 {
        // NOTICE: Really needs validation xD
        // oam_read and oam_write might be accessed from the DMA, which won't be using 0xFE00-offset
        if 0xFE00 <= address {
            address -= 0xFE00;
        }

        let entry = address % 4;

        let oam_index = address / 4;

        match entry {
            0 => self.oam_ram[oam_index as usize].y,
            1 => self.oam_ram[oam_index as usize].x,
            2 => self.oam_ram[oam_index as usize].tile,
            3 => self.oam_ram[oam_index as usize].flag,
            _ => unreachable!()
        }
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        // Offset already added
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }

    pub fn tick(&mut self) {
        self.line_ticks += 1;

        println!("PPU.tick()!");
        let lcd = LCD.write().unwrap();
        match lcd.status_mode() {
            LCDMode::OAM => self.mode_oam(lcd),
            LCDMode::XFER => self.mode_xfer(lcd),
            LCDMode::VBlank => self.mode_vblank(lcd),
            LCDMode::HBlank => self.mode_hblank(lcd),
        }
    }
}