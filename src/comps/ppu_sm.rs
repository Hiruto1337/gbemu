use std::sync::{RwLockWriteGuard, RwLock};

use super::{ppu::{PPUContext, TICKS_PER_LINE, LINES_PER_FRAME, Y_RES}, lcd::{LCDMode, LCDContext, StatusSource}, cpu::CPU, interrupts::InterruptType, common::{TIME, delay}};

const TARGET_FRAME_TIME: u32 = 1000 / 60; // 60 frames per second

static PREV_FRAME_TIME: RwLock<u32> = RwLock::new(0);
static START_TIMER: RwLock<u32> = RwLock::new(0);
static FRAME_COUNT: RwLock<u32> = RwLock::new(0);

impl PPUContext {
    fn increment_line_y(&mut self, lcd: &mut RwLockWriteGuard<LCDContext>) {
        lcd.line_y += 1;

        if lcd.line_y == lcd.line_y_compare {
            lcd.status_line_y_compare_set(true);

            if lcd.status_stat_int(StatusSource::LYC) {
                CPU.write().unwrap().request_interrupt(InterruptType::LCDStat);
            }
        } else {
            lcd.status_line_y_compare_set(false);
        }
    }

    pub fn mode_oam(&self, mut lcd: RwLockWriteGuard<LCDContext>) {
        if 80 <= self.line_ticks {
            lcd.status_mode_set(LCDMode::XFER);
        }
    }

    pub fn mode_xfer(&self, mut lcd: RwLockWriteGuard<LCDContext>) {
        if 80 + 172 <= self.line_ticks {
            lcd.status_mode_set(LCDMode::HBlank);
        }
    }

    pub fn mode_hblank(&mut self, mut lcd: RwLockWriteGuard<LCDContext>) {
        if TICKS_PER_LINE <= self.line_ticks { // End of line reached
            self.increment_line_y(&mut lcd);

            if Y_RES <= lcd.line_y { // End of frame reached
                lcd.status_mode_set(LCDMode::VBlank);

                let mut cpu = CPU.write().unwrap();
                cpu.request_interrupt(InterruptType::VBlank);

                if lcd.status_stat_int(StatusSource::VBlank) {
                    cpu.request_interrupt(InterruptType::LCDStat);
                }

                self.current_frame += 1;

                // Calculate FPS
                let end = TIME.read().unwrap().unwrap().elapsed().as_millis() as u32;
                let frame_time = end - *PREV_FRAME_TIME.read().unwrap();

                if frame_time < TARGET_FRAME_TIME {
                    delay((TARGET_FRAME_TIME - frame_time) as u64);
                }

                if 1000 <= end - *START_TIMER.read().unwrap() {
                    let fps = *FRAME_COUNT.read().unwrap();
                    *START_TIMER.write().unwrap() = end;
                    *FRAME_COUNT.write().unwrap() = 0;

                    println!("FPS: {fps}");
                }

                *FRAME_COUNT.write().unwrap() += 1;
                *PREV_FRAME_TIME.write().unwrap() = TIME.read().unwrap().unwrap().elapsed().as_millis() as u32;
            } else {
                lcd.status_mode_set(LCDMode::OAM);
            }

            self.line_ticks = 0;
        }
    }

    pub fn mode_vblank(&mut self, mut lcd: RwLockWriteGuard<LCDContext>) {
        if TICKS_PER_LINE <= self.line_ticks {
            self.increment_line_y(&mut lcd);

            if LINES_PER_FRAME <= lcd.line_y { // End of frame reached
                lcd.status_mode_set(LCDMode::OAM);
                lcd.line_y = 0;
            }

            self.line_ticks = 0;
        }
    }
}