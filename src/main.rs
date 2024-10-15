#[path = "instructions/cbprefixed.rs"]
mod cbprefixed;
#[path = "instructions/unprefixed.rs"]
mod unprefixed;

mod cpu;
mod device;
mod memory;
mod ppu;
mod registers;

use device::Device;

use std::env;
use std::path::PathBuf;

use minifb::{Key, Window, WindowOptions};

const PATH_TO_ROM: &str = "roms/wordzap.gb";

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const SCALE: usize = 4;

fn main() {
    let mut gb = Device::new();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rom_path = PathBuf::from(manifest_dir).join(PATH_TO_ROM);

    gb.load_instructions(rom_path.to_str().unwrap());

    let mut window = Window::new(
        "Game Boy Emulator",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    loop {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }

        gb.cpu.cycle();

        if gb.ppu.borrow().frame_ready {
            let scaled_buffer =
                scale_framebuffer(&gb.ppu.borrow().framebuffer, WIDTH, HEIGHT, SCALE);

            window
                .update_with_buffer(&scaled_buffer, WIDTH * SCALE, HEIGHT * SCALE)
                .unwrap();

            gb.ppu.borrow_mut().frame_ready = false;
        } else {
            window.update();
        }
    }
}

fn scale_framebuffer(framebuffer: &[u32], width: usize, height: usize, scale: usize) -> Vec<u32> {
    let scaled_width = width * scale;
    let scaled_height = height * scale;
    let mut scaled_buffer = vec![0; scaled_width * scaled_height];

    for y in 0..height {
        for x in 0..width {
            let color = framebuffer[y * width + x];
            let base_x = x * scale;
            let base_y = y * scale;

            for dy in 0..scale {
                for dx in 0..scale {
                    let sx = base_x + dx;
                    let sy = base_y + dy;
                    scaled_buffer[sy * scaled_width + sx] = color;
                }
            }
        }
    }

    scaled_buffer
}
