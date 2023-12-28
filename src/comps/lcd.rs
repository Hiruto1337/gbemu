use std::sync::RwLock;

use super::{common::{bit, bit_set, COLORS}, dma::DMA};

pub struct LCDContext {
    // Registers,
    pub control: u8,        // 0xFF40
    pub status: u8,         // 0xFF41
    pub scroll_y: u8,       // 0xFF42
    pub scroll_x: u8,       // 0xFF43
    pub line_y: u8,         // 0xFF44
    pub line_y_compare: u8, // 0xFF45
    pub dma: u8,            // 0xFF46
    pub bg_palette: u8,     // 0xFF47
    pub obj1_palette: u8,   // 0xFF48
    pub obj2_palette: u8,   // 0xFF49
    pub window_y: u8,       // 0xFF4A
    pub window_x: u8,       // 0xFF4B

    // Other data
    pub bg_colors: [u32; 4],
    pub sprite1_colors: [u32; 4],
    pub sprite2_colors: [u32; 4],
}

pub static LCD: RwLock<LCDContext> = RwLock::new(LCDContext {
    control: 0x91,
    status: 0b10, // NOTICE: Starts in OAM mode
    scroll_y: 0,
    scroll_x: 0,
    line_y: 0,
    line_y_compare: 0,
    dma: 0,
    bg_palette: 0xFC,
    obj1_palette: 0xFF,
    obj2_palette: 0xFF,
    window_y: 0,
    window_x: 0,
    bg_colors:      [COLORS[0], COLORS[1], COLORS[2], COLORS[3]],
    sprite1_colors: [COLORS[0], COLORS[1], COLORS[2], COLORS[3]],
    sprite2_colors: [COLORS[0], COLORS[1], COLORS[2], COLORS[3]],
});

impl LCDContext {
    pub fn read(&self, address: u16) -> u8 { // NOTICE: NEEDS COLOSSAL REFACTORING
        match address {
            0xFF40 => self.control,
            0xFF41 => self.status,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.line_y,
            0xFF45 => self.line_y_compare,
            0xFF46 => self.dma,
            0xFF47 => self.bg_palette,
            0xFF48 => self.obj1_palette,
            0xFF49 => self.obj2_palette,
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,

            // Background colors
            0xFF4C => (self.bg_colors[0] >> 24) as u8,
            0xFF4D => (self.bg_colors[0] >> 16) as u8,
            0xFF4E => (self.bg_colors[0] >> 8) as u8,
            0xFF4F => self.bg_colors[0] as u8,

            0xFF50 => (self.bg_colors[1] >> 24) as u8,
            0xFF51 => (self.bg_colors[1] >> 16) as u8,
            0xFF52 => (self.bg_colors[1] >> 8) as u8,
            0xFF53 => self.bg_colors[1] as u8,

            0xFF54 => (self.bg_colors[2] >> 24) as u8,
            0xFF55 => (self.bg_colors[2] >> 16) as u8,
            0xFF56 => (self.bg_colors[2] >> 8) as u8,
            0xFF57 => self.bg_colors[2] as u8,

            0xFF58 => (self.bg_colors[3] >> 24) as u8,
            0xFF59 => (self.bg_colors[3] >> 16) as u8,
            0xFF5A => (self.bg_colors[3] >> 8) as u8,
            0xFF5B => self.bg_colors[3] as u8,

            // Sprite 1 colors
            0xFF5C => (self.sprite1_colors[0] >> 24) as u8,
            0xFF5D => (self.sprite1_colors[0] >> 16) as u8,
            0xFF5E => (self.sprite1_colors[0] >> 8) as u8,
            0xFF5F => self.sprite1_colors[0] as u8,

            0xFF60 => (self.sprite1_colors[1] >> 24) as u8,
            0xFF61 => (self.sprite1_colors[1] >> 16) as u8,
            0xFF62 => (self.sprite1_colors[1] >> 8) as u8,
            0xFF63 => self.sprite1_colors[1] as u8,

            0xFF64 => (self.sprite1_colors[2] >> 24) as u8,
            0xFF65 => (self.sprite1_colors[2] >> 16) as u8,
            0xFF66 => (self.sprite1_colors[2] >> 8) as u8,
            0xFF67 => self.sprite1_colors[2] as u8,

            0xFF68 => (self.sprite1_colors[3] >> 24) as u8,
            0xFF69 => (self.sprite1_colors[3] >> 16) as u8,
            0xFF6A => (self.sprite1_colors[3] >> 8) as u8,
            0xFF6B => self.sprite1_colors[3] as u8,

            // Sprite 2 colors
            0xFF6C => (self.sprite2_colors[0] >> 24) as u8,
            0xFF6D => (self.sprite2_colors[0] >> 16) as u8,
            0xFF6E => (self.sprite2_colors[0] >> 8) as u8,
            0xFF6F => self.sprite2_colors[0] as u8,

            0xFF70 => (self.sprite2_colors[1] >> 24) as u8,
            0xFF71 => (self.sprite2_colors[1] >> 16) as u8,
            0xFF72 => (self.sprite2_colors[1] >> 8) as u8,
            0xFF73 => self.sprite2_colors[1] as u8,

            0xFF74 => (self.sprite2_colors[2] >> 24) as u8,
            0xFF75 => (self.sprite2_colors[2] >> 16) as u8,
            0xFF76 => (self.sprite2_colors[2] >> 8) as u8,
            0xFF77 => self.sprite2_colors[2] as u8,

            0xFF78 => (self.sprite2_colors[3] >> 24) as u8,
            0xFF79 => (self.sprite2_colors[3] >> 16) as u8,
            0xFF7A => (self.sprite2_colors[3] >> 8) as u8,
            0xFF7B => self.sprite2_colors[3] as u8,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, address: u16, value: u8) { // NOTICE: NEEDS COLOSSAL REFACTORING
        match address {
            0xFF40 => self.control = value,
            0xFF41 => self.status = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.line_y = value,
            0xFF45 => self.line_y_compare = value,
            0xFF46 => {
                // NOTICE: Is this implemented correctly?
                self.dma = value;
                DMA.write().unwrap().start(value);
            },
            0xFF47 => {
                self.bg_palette = value;
                self.update_palette(value, 0);
            },
            0xFF48 => {
                self.obj1_palette = value;
                self.update_palette(value & (!0b11), 1);
            },
            0xFF49 => {
                self.obj2_palette = value;
                self.update_palette(value & (!0b11), 2);
            },
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,

            // Background colors
            0xFF4C => {
                self.bg_colors[0] &= !(0xFF << 24);
                self.bg_colors[0] |= (value as u32) << 24;
            },
            0xFF4D => {
                self.bg_colors[0] &= !(0xFF << 16);
                self.bg_colors[0] |= (value as u32) << 16;
            },
            0xFF4E => {
                self.bg_colors[0] &= !(0xFF << 8);
                self.bg_colors[0] |= (value as u32) << 8;
            },
            0xFF4F => {
                self.bg_colors[0] &= !0xFF;
                self.bg_colors[0] |= value as u32;
            },

            0xFF50 => {
                self.bg_colors[1] &= !(0xFF << 24);
                self.bg_colors[1] |= (value as u32) << 24;
            },
            0xFF51 => {
                self.bg_colors[1] &= !(0xFF << 16);
                self.bg_colors[1] |= (value as u32) << 16;
            },
            0xFF52 => {
                self.bg_colors[1] &= !(0xFF << 8);
                self.bg_colors[1] |= (value as u32) << 8;
            },
            0xFF53 => {
                self.bg_colors[1] &= !0xFF;
                self.bg_colors[1] |= value as u32;
            },

            0xFF54 => {
                self.bg_colors[2] &= !(0xFF << 24);
                self.bg_colors[2] |= (value as u32) << 24;
            },
            0xFF55 => {
                self.bg_colors[2] &= !(0xFF << 16);
                self.bg_colors[2] |= (value as u32) << 16;
            },
            0xFF56 => {
                self.bg_colors[2] &= !(0xFF << 8);
                self.bg_colors[2] |= (value as u32) << 8;
            },
            0xFF57 => {
                self.bg_colors[2] &= !0xFF;
                self.bg_colors[2] |= value as u32;
            },

            0xFF58 => {
                self.bg_colors[3] &= !(0xFF << 24);
                self.bg_colors[3] |= (value as u32) << 24;
            },
            0xFF59 => {
                self.bg_colors[3] &= !(0xFF << 16);
                self.bg_colors[3] |= (value as u32) << 16;
            },
            0xFF5A => {
                self.bg_colors[3] &= !(0xFF << 8);
                self.bg_colors[3] |= (value as u32) << 8;
            },
            0xFF5B => {
                self.bg_colors[3] &= !0xFF;
                self.bg_colors[3] |= value as u32;
            },

            // Sprite 1 colors
            0xFF5C => {
                self.sprite1_colors[0] &= !(0xFF << 24);
                self.sprite1_colors[0] |= (value as u32) << 24;
            },
            0xFF5D => {
                self.sprite1_colors[0] &= !(0xFF << 16);
                self.sprite1_colors[0] |= (value as u32) << 16;
            },
            0xFF5E => {
                self.sprite1_colors[0] &= !(0xFF << 8);
                self.sprite1_colors[0] |= (value as u32) << 8;
            },
            0xFF5F => {
                self.sprite1_colors[0] &= !0xFF;
                self.sprite1_colors[0] |= value as u32;
            },

            0xFF60 => {
                self.sprite1_colors[1] &= !(0xFF << 24);
                self.sprite1_colors[1] |= (value as u32) << 24;
            },
            0xFF61 => {
                self.sprite1_colors[1] &= !(0xFF << 16);
                self.sprite1_colors[1] |= (value as u32) << 16;
            },
            0xFF62 => {
                self.sprite1_colors[1] &= !(0xFF << 8);
                self.sprite1_colors[1] |= (value as u32) << 8;
            },
            0xFF63 => {
                self.sprite1_colors[1] &= !0xFF;
                self.sprite1_colors[1] |= value as u32;
            },

            0xFF64 => {
                self.sprite1_colors[2] &= !(0xFF << 24);
                self.sprite1_colors[2] |= (value as u32) << 24;
            },
            0xFF65 => {
                self.sprite1_colors[2] &= !(0xFF << 16);
                self.sprite1_colors[2] |= (value as u32) << 16;
            },
            0xFF66 => {
                self.sprite1_colors[2] &= !(0xFF << 8);
                self.sprite1_colors[2] |= (value as u32) << 8;
            },
            0xFF67 => {
                self.sprite1_colors[2] &= !0xFF;
                self.sprite1_colors[2] |= value as u32;
            },

            0xFF68 => {
                self.sprite1_colors[3] &= !(0xFF << 24);
                self.sprite1_colors[3] |= (value as u32) << 24;
            },
            0xFF69 => {
                self.sprite1_colors[3] &= !(0xFF << 16);
                self.sprite1_colors[3] |= (value as u32) << 16;
            },
            0xFF6A => {
                self.sprite1_colors[3] &= !(0xFF << 8);
                self.sprite1_colors[3] |= (value as u32) << 8;
            },
            0xFF6B => {
                self.sprite1_colors[3] &= !0xFF;
                self.sprite1_colors[3] |= value as u32;
            },

            // Sprite 2 colors
            0xFF6C => {
                self.sprite2_colors[0] &= !(0xFF << 24);
                self.sprite2_colors[0] |= (value as u32) << 24;
            },
            0xFF6D => {
                self.sprite2_colors[0] &= !(0xFF << 16);
                self.sprite2_colors[0] |= (value as u32) << 16;
            },
            0xFF6E => {
                self.sprite2_colors[0] &= !(0xFF << 8);
                self.sprite2_colors[0] |= (value as u32) << 8;
            },
            0xFF6F => {
                self.sprite2_colors[0] &= !0xFF;
                self.sprite2_colors[0] |= value as u32;
            },

            0xFF70 => {
                self.sprite2_colors[1] &= !(0xFF << 24);
                self.sprite2_colors[1] |= (value as u32) << 24;
            },
            0xFF71 => {
                self.sprite2_colors[1] &= !(0xFF << 16);
                self.sprite2_colors[1] |= (value as u32) << 16;
            },
            0xFF72 => {
                self.sprite2_colors[1] &= !(0xFF << 8);
                self.sprite2_colors[1] |= (value as u32) << 8;
            },
            0xFF73 => {
                self.sprite2_colors[1] &= !0xFF;
                self.sprite2_colors[1] |= value as u32;
            },

            0xFF74 => {
                self.sprite2_colors[2] &= !(0xFF << 24);
                self.sprite2_colors[2] |= (value as u32) << 24;
            },
            0xFF75 => {
                self.sprite2_colors[2] &= !(0xFF << 16);
                self.sprite2_colors[2] |= (value as u32) << 16;
            },
            0xFF76 => {
                self.sprite2_colors[2] &= !(0xFF << 8);
                self.sprite2_colors[2] |= (value as u32) << 8;
            },
            0xFF77 => {
                self.sprite2_colors[2] &= !0xFF;
                self.sprite2_colors[2] |= value as u32;
            },

            0xFF78 => {
                self.sprite2_colors[3] &= !(0xFF << 24);
                self.sprite2_colors[3] |= (value as u32) << 24;
            },
            0xFF79 => {
                self.sprite2_colors[3] &= !(0xFF << 16);
                self.sprite2_colors[3] |= (value as u32) << 16;
            },
            0xFF7A => {
                self.sprite2_colors[3] &= !(0xFF << 8);
                self.sprite2_colors[3] |= (value as u32) << 8;
            },
            0xFF7B => {
                self.sprite2_colors[3] &= !0xFF;
                self.sprite2_colors[3] |= value as u32;
            },
            _ => unreachable!()
        }
    }

    fn update_palette(&mut self, palette_data: u8, pal: u8) {
        let colors = match pal {
            0 => &mut self.bg_colors,
            1 => &mut self.sprite1_colors,
            2 => &mut self.sprite2_colors,
            _ => unreachable!()
        };

        colors[0] = COLORS[(palette_data & 0b11) as usize];
        colors[1] = COLORS[((palette_data >> 2) & 0b11) as usize];
        colors[2] = COLORS[((palette_data >> 4) & 0b11) as usize];
        colors[3] = COLORS[((palette_data >> 6) & 0b11) as usize];
    }

    // Control
    pub fn control_bgw_enable(&self) -> bool {
        bit(self.control, 0)
    }

    pub fn control_obj_enable(&self) -> bool {
        bit(self.control, 1)
    }

    pub fn control_obj_height(&self) -> u8 {
        match bit(self.control, 2) {
            true => 16,
            false => 8,
        }
    }

    pub fn control_bg_map_area(&self) -> u16 {
        match bit(self.control, 3) {
            true => 0x9C00,
            false => 0x9800,
        }
    }

    pub fn control_bgw_data_area(&self) -> u16 {
        match bit(self.control, 4) {
            true => 0x8000,
            false => 0x8800,
        }
    }

    pub fn control_win_enable(&self) -> bool {
        bit(self.control, 5)
    }

    pub fn control_win_map_area(&self) -> u16 {
        match bit(self.control, 6) {
            true => 0x9C00,
            false => 0x9800,
        }
    }

    pub fn control_lcd_enable(&self) -> bool {
        bit(self.control, 7)
    }

    // Status
    pub fn status_mode(&self) -> LCDMode {
        LCDMode::from(self.status & 0b11)
    }

    pub fn status_mode_set(&mut self, mode: LCDMode) {
        let mode = mode as u8;
        self.status &= !0b11;
        self.status |= mode;
    }

    pub fn status_line_y_compare(&self) -> bool {
        bit(self.status, 2)
    }

    pub fn status_line_y_compare_set(&mut self, b: bool) {
        bit_set(&mut self.status, 2, b);
    }

    pub fn status_stat_int(&self, source: StatusSource) -> bool {
        // NOTICE: Should work
        self.status & source as u8 != 0
    }
}

pub enum LCDMode {
    HBlank,
    VBlank,
    OAM,
    XFER, // Transfer
}

impl From<u8> for LCDMode {
    fn from(value: u8) -> Self {
        match value {
            0 => LCDMode::HBlank,
            1 => LCDMode::VBlank,
            2 => LCDMode::OAM,
            3 => LCDMode::XFER,
            _ => unreachable!()
        }
    }
}

pub enum StatusSource {
    HBlank = 1 << 3,
    VBlank = 1 << 4,
    OAM = 1 << 5,
    LYC = 1 << 6
}
