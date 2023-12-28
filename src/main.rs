use std::time::{Duration, Instant};

use gbemu::comps::{bus::bus_read, cart::CART, cpu::CPU, emu::EMULATOR, ppu::PPU, timer::TIMER, common::{COLORS, TIME}};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

pub const SCALE: u16 = 2;

fn main() {
    // Initialize cartridge
    let args: Vec<String> = std::env::args().collect();
    CART.write().unwrap().load(args);

    // Initialize PPU
    PPU.write().unwrap().init();

    // Initialize SDL and time
    let sdl_context = sdl2::init().unwrap();
    *TIME.write().unwrap() = Some(Instant::now());
    let video_subsystem = sdl_context.video().unwrap();

    let debug_window = video_subsystem
        .window(
            "Debug",
            16 * 8 * SCALE as u32 + 16 * SCALE as u32,
            32 * 8 * SCALE as u32 + 64 * SCALE as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut debug_canvas = debug_window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize CPU on separate thread
    std::thread::spawn(|| {
        TIMER.write().unwrap().div = 0xABCC;

        while EMULATOR.read().unwrap().running {
            if EMULATOR.read().unwrap().paused {
                delay(10);
                continue;
            }

            CPU.write().unwrap().step();
        }
    });

    let mut prev_frame = 0;

    // Update UI
    while !EMULATOR.read().unwrap().die {
        handle_events(&mut event_pump);

        if prev_frame != PPU.read().unwrap().current_frame {
            update_debug_window(&mut debug_canvas);
        }

        prev_frame = PPU.read().unwrap().current_frame;
    }
}

pub fn handle_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => EMULATOR.write().unwrap().die = true,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => EMULATOR.write().unwrap().die = true,
            _ => {}
        }
    }
}

pub fn display_tile(
    debug_canvas: &mut Canvas<Window>,
    start_location: u16,
    tile_num: u16,
    x: u16,
    y: u16,
) {
    let mut rect: Rect;

    for line in (0..16).step_by(2) {
        let cpu = CPU.read().unwrap();
        let byte1 = bus_read(&cpu, start_location + (tile_num * 16) + line);
        let byte2 = bus_read(&cpu, start_location + (tile_num * 16) + line + 1);
        drop(cpu);

        for bit in (0..8).rev() {
            let hi_bit = ((byte1 >> bit) & 1) << 1;
            let lo_bit = (byte2 >> bit) & 1;

            let color = hi_bit | lo_bit;
            
            // NOTICE: This seems kinda fucked
            debug_canvas.set_draw_color(Color::from_u32(&PixelFormatEnum::ABGR8888.try_into().unwrap(), COLORS[color as usize]));

            rect = Rect::new(
                (x as i32 + (7 - bit) as i32) * SCALE as i32,
                (y as i32 + (line as i32 / 2)) * SCALE as i32,
                SCALE.into(),
                SCALE.into(),
            );

            debug_canvas.draw_rect(rect).unwrap();
        }
    }
}

pub fn update_debug_window(debug_canvas: &mut Canvas<Window>) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let (width, height) = debug_canvas.output_size().unwrap();

    debug_canvas.set_draw_color(Color::RGB(11, 11, 11));
    debug_canvas
        .fill_rect(Rect::new(0, 0, width, height))
        .unwrap();

    let address = 0x8000;

    // 384 tiles -> 1 tile is 24 x 16
    for y in 0..24 {
        for x in 0..16 {
            display_tile(
                debug_canvas,
                address,
                tile_num,
                x_draw + x * SCALE,
                y_draw + y * SCALE,
            );
            x_draw += 4 * SCALE;
            tile_num += 1;
        }
        y_draw += 4 * SCALE;
        x_draw = 0;
    }

    debug_canvas.present();
}

pub fn delay(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
