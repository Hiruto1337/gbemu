use std::time::Duration;

use gbemu::comps::{
    cart::CART,
    cpu::{CPUContext, Registers},
    emu::EMULATOR,
    instructions::INSTRUCTIONS,
    ppu::PPU,
    timer::TIMER, bus::bus_read,
};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

pub const SCALE: u16 = 2;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    CART.lock().unwrap().load(args);

    // PPU.lock().unwrap().init();

    // Initialize SDL and fonts
    let sdl_context = sdl2::init().unwrap();
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

    // Initialize CPU on separate thread
    std::thread::spawn(|| {
        let mut cpu = CPUContext {
            registers: Registers {
                pc: 0x100,
                sp: 0xFFFE,
                a: 0x01,
                f: 0xB0,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
            },
            ie_register: 0,
            int_flags: 0,
            int_master_enabled: false,
            enabling_ime: false,

            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: &INSTRUCTIONS[0],
            halted: false,
            stepping: true,
        };

        TIMER.lock().unwrap().div = 0xABCC;

        while EMULATOR.lock().unwrap().running {
            if EMULATOR.lock().unwrap().paused {
                delay(10);
                continue;
            }

            cpu.step();
        }
    });

    let mut debug_canvas = debug_window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    while !EMULATOR.lock().unwrap().die {
        delay(33);
        handle_events(&mut event_pump);
        update_debug_window(&mut debug_canvas);
    }
}

pub fn handle_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => EMULATOR.lock().unwrap().die = true,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => EMULATOR.lock().unwrap().die = true,
            _ => {}
        }
    }
}

const TILE_COLORS: [Color; 4] = [
    Color::RGB(0xFF, 0xFF, 0xFF),
    Color::RGB(0xAA, 0xAA, 0xAA),
    Color::RGB(0x55, 0x55, 0x55),
    Color::RGB(0x00, 0x00, 0x00),
];

pub fn display_tile(
    debug_canvas: &mut Canvas<Window>,
    start_location: u16,
    tile_num: u16,
    x: u16,
    y: u16,
) {
    let mut rect: Rect;

    let ppu = PPU.lock().unwrap();

    for line in (0..16).step_by(2) {
        let byte1 = bus_read(start_location + (tile_num * 16) + line);
        let byte2 = bus_read(start_location + (tile_num * 16) + line + 1);

        for bit in (0..8).rev() {
            let hi_bit = ((byte1 >> bit) & 1) << 1;
            let lo_bit = (byte2 >> bit) & 1;

            let color = hi_bit | lo_bit;
            debug_canvas.set_draw_color(TILE_COLORS[color as usize]);

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
            ); // NOTICE: Scaling
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
