use super::{
    bus::bus_read,
    cpu::CPUContext,
    lcd::LCDContext,
    ppu::{FetchState, PPUContext, X_RES},
};

impl PPUContext {
    pub fn pipeline_process(&mut self, cpu: &mut CPUContext, lcd: &mut LCDContext) {
        self.pfc.map_y = lcd.line_y.wrapping_add(lcd.scroll_y); // NOTICE: wrapping_add()
        self.pfc.map_x = self.pfc.fetch_x + lcd.scroll_x;
        self.pfc.tile_y = ((lcd.line_y.wrapping_add(lcd.scroll_y)) % 8) * 2; // NOTICE: wrapping_add()

        // Every other tick
        if self.line_ticks & 1 == 0 {
            self.pipeline_fetch(cpu, lcd);
        }

        // Every tick
        self.pipeline_push_pixel(lcd);
    }

    fn pipeline_push_pixel(&mut self, lcd: &mut LCDContext) {
        if 8 < self.pfc.pixel_fifo.len() {
            let pixel_data = self.pfc.pixel_fifo.pop_front().unwrap();

            if lcd.scroll_x % 8 <= self.pfc.line_x {
                let index = self.pfc.pushed_x as usize + (lcd.line_y as usize * X_RES as usize);
                self.frame_buffer[index] = pixel_data;

                self.pfc.pushed_x += 1;
            }

            self.pfc.line_x += 1;
        }
    }

    fn pipeline_fetch(&mut self, cpu: &mut CPUContext, lcd: &LCDContext) {
        match self.pfc.cur_fetch_state {
            FetchState::TILE => {
                if lcd.control_bgw_enable() {
                    // Only if background/window is enabled
                    let address = lcd.control_bg_map_area()
                        + self.pfc.map_x as u16 / 8
                        + self.pfc.map_y as u16 / 8 * 32;

                    self.pfc.bgw_fetch_data[0] = bus_read(cpu, self, address);

                    if lcd.control_bgw_data_area() == 0x8800 {
                        self.pfc.bgw_fetch_data[0] = self.pfc.bgw_fetch_data[0].wrapping_add(128); // NOTICE: wrapping_add()
                    }
                }

                self.pfc.cur_fetch_state = FetchState::DATA0;
                self.pfc.fetch_x += 8;
            }
            FetchState::DATA0 => {
                self.pfc.bgw_fetch_data[1] = bus_read(
                    cpu,
                    self,
                    lcd.control_bgw_data_area()
                        + self.pfc.bgw_fetch_data[0] as u16 * 16
                        + self.pfc.tile_y as u16,
                );

                self.pfc.cur_fetch_state = FetchState::DATA1;
            }
            FetchState::DATA1 => {
                self.pfc.bgw_fetch_data[2] = bus_read(
                    cpu,
                    self,
                    lcd.control_bgw_data_area()
                        + self.pfc.bgw_fetch_data[0] as u16 * 16
                        + self.pfc.tile_y as u16
                        + 1,
                );

                self.pfc.cur_fetch_state = FetchState::SLEEP;
            }
            FetchState::SLEEP => self.pfc.cur_fetch_state = FetchState::PUSH,
            FetchState::PUSH => {
                if self.pipeline_fifo_add(lcd) {
                    self.pfc.cur_fetch_state = FetchState::TILE;
                }
            }
        }
    }

    fn pipeline_fifo_add(&mut self, lcd: &LCDContext) -> bool {
        // If pixel_fifo is greater than 8, it's full and we can't add it
        if 8 < self.pfc.pixel_fifo.len() {
            return false;
        }

        let x = self.pfc.fetch_x as i16 - (8 - (lcd.scroll_x as i16 % 8)); // NOTICE: Might be weird types?

        for i in 0..8 {
            let bit = 7 - i;

            let hi = ((self.pfc.bgw_fetch_data[2] >> bit) & 1) << 1;
            let lo = (self.pfc.bgw_fetch_data[1] >> bit) & 1;

            let color = lcd.bg_colors[(hi | lo) as usize];

            if 0 <= x {
                self.pfc.pixel_fifo.push_back(color);
                self.pfc.fifo_x += 1;
            }
        }

        true
    }

    pub fn pipeline_fifo_reset(&mut self) {
        self.pfc.pixel_fifo.drain(..); // NOTICE: Pretty sure this should work
    }
}
