mod chip8;
mod keyboard;
mod monitor;
mod speaker;

extern crate sdl2;

use chip8::Chip8;
use keyboard::*;
use monitor::*;

use sdl2::event::{Event, Event::KeyDown};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, Instant};
use std::{fs, path::Path};

const SCREEN_W: u32 = 1280;
const SCREEN_H: u32 = 720;
const FPS: u64 = 60;
// Divide it by a milisecond to get the interval. Basically pin the chip8 cycles
// to execute every 16 miliseconds
const FPS_INTERVAL: Duration = Duration::from_millis(1000 / FPS);

fn calculate_delta(start: Instant) -> Duration {
    Instant::now().duration_since(start)
}

fn display_metrics(
    canvas: &mut Canvas<Window>,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    metrics: &str,
) {
    let surface = font
        .render(metrics)
        .blended(Color::RGBA(255, 255, 255, 255))
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let target = Rect::new(0, 720 - 50, 1280, 50);
    canvas.copy(&texture, None, Some(target)).unwrap();
}

// Takes the buffer of the monitor and draws it to the canvas
fn draw(screen: [u8; 2048], canvas: &mut Canvas<Window>, offset_x: u32, offset_y: u32) {
    let mut rect = Rect::new(0, 0, SCALE as u32, SCALE as u32);
    let mut y = 0;
    for (index, px) in screen.iter().enumerate() {
        if index % COLS == 0 && index > 0 {
            y += 1;
        }
        if *px == 1 {
            rect.reposition((
                ((index % 64) * SCALE) as i32 + offset_x as i32,
                (y * SCALE) as i32 + offset_y as i32,
            ));
            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(rect).unwrap();
        }
    }
}

pub fn main() {
    // Set the audio and video subsystems
    let sdl_context = sdl2::init().unwrap();
    let ttl_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let font = ttl_context
        .load_font(Path::new("./OpenSans-Regular.ttf"), 128)
        .unwrap();
    // Init chip 8 and components
    let monitor = Monitor::new_default();
    let audio_device = speaker::init_speaker(audio_subsystem);
    let mut chip8 = Chip8::new(monitor);

    // Generate the window
    let (c8_width, c8_height) = chip8.monitor.get_scaled_res();
    let window = video_subsystem
        .window("CHIP-8", SCREEN_W, SCREEN_H)
        .position_centered()
        .build()
        .unwrap();

    // Set canvas, texture creator and event pump
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // Read the ROM and load it into chip8
    let rom = fs::read(Path::new("./roms/Framed.ch8")).expect("Couldn't read ROM");
    chip8.load_sprites();
    chip8.load_program(&rom);

    let mut start = Instant::now();
    // The loop
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                // Chip 8 related keys
                KeyDown {
                    keycode: Some(Keycode::Kp0),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Zero));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp1),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Seven));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp2),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Eight));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp3),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Nine));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp4),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Four));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp5),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Five));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp6),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Six));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp7),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::One));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp8),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Two));
                }
                KeyDown {
                    keycode: Some(Keycode::Kp9),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::Three));
                }
                KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::A));
                }
                KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::B));
                }
                KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::C));
                }
                KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::D));
                }
                KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::E));
                }
                KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    chip8.keyboard.press_key(Some(Chip8Key::F));
                }
                // Esc to exit
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Utility:
                // Run the chip8 cycle only if space is held down
                // Event::KeyDown {
                // keycode: Some(Keycode::Space),
                // ..
                // } => {
                // chip8.debug_cycle();
                // }
                _ => {}
            }
        }
        // Cycle the chip8
        if calculate_delta(start) >= FPS_INTERVAL {
            chip8.cycle();
            start = Instant::now();
        }
        // Play sound
        if chip8.check_sound() {
            audio_device.resume();
        } else {
            audio_device.pause();
        }
        // Update metrics
        display_metrics(
            &mut canvas,
            &font,
            &texture_creator,
            &chip8.get_metrics()[..],
        );
        // Draw
        let screen = chip8.monitor.get_buffer();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw(
            screen,
            &mut canvas,
            (SCREEN_W - c8_width) / 2,
            (SCREEN_H - c8_height) / 2,
        );
        canvas.present();
    }
}

#[allow(dead_code)]
fn test_draw(screen: [u8; 2048], canvas: &mut Canvas<Window>) {
    let mut rect = Rect::new(0, 0, 5 as u32, 5 as u32);
    let mut y = 0;
    for (i, px) in screen.iter().enumerate() {
        if i % COLS == 0 && i > 0 {
            println!("INDEX: {}", i);
            y += 1;
        }
        if *px == 1 {
            println!("X: {}", i % 64);
            rect.reposition((((i % COLS) * SCALE) as i32, (y * SCALE) as i32));
            canvas.fill_rect(rect).unwrap();
            canvas.draw_rect(rect).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converison_sanity() {
        assert_eq!((2 * SCALE) as i32, ((2 * SCALE) as usize) as i32)
    }
}
