extern crate sdl;
use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use sdl::Rect;

use core::time;
use std::time::Duration;
use std::{
    time::SystemTime,
    thread
};

use rand::{self, Rng};

use clap::Parser;

use std;

mod chip8;

use chip8::vm::{
    KeyboardHandler,
    PixelHandler, RandomHandler,
};

struct BasicPixelHandler<'a> {
    screen: &'a mut sdl::video::Surface,
}

impl<'a> PixelHandler for BasicPixelHandler<'a> {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let rect = Rect {
            x: (x as isize * SIZE) as i16,
            y: (y as isize * SIZE) as i16,
            w: SIZE as u16,
            h: SIZE as u16,
        };

        let color = if on { sdl::video::RGB(255, 255, 255) } else { sdl::video::RGB(0, 0, 0) };
        self.screen.fill_rect(Some(rect), color);
    }
}

struct BasicKeyboardHandler {
    status: [bool; 16],
}

impl KeyboardHandler for BasicKeyboardHandler {
    fn is_pressed(&mut self, key: u8) -> bool {
        if key > 16 {
            return false
        }
        self.status[key as usize]
    }
}

struct BasicRandomHandler {
    rng: rand::rngs::ThreadRng
}

impl RandomHandler for BasicRandomHandler {
    fn random(&mut self) -> u8 {
        self.rng.gen()
    }
}

static SIZE: isize = 10;
static FPS: u128 = 60;
static FREQUENCY: u32 = 500;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to open
    #[arg(short, long)]
    file: String,
}


#[cfg(test)]
mod tests {
    use crate::run_emulator;

    #[test]
    fn chip8logo() {
        run_emulator("./roms/tests/1-chip8-logo.ch8".to_string(), true);
    }

    #[test]
    fn ibmlogo() {
        run_emulator("./roms/tests/2-ibm-logo.ch8".to_string(), true);
    }

    #[test]
    fn corax() {
        run_emulator("./roms/tests/3-corax+.ch8".to_string(), true);
    }

    #[test]
    fn flags() {
        run_emulator("./roms/tests/4-flags.ch8".to_string(), true);
    }

    #[test]
    fn quirks() {
        run_emulator("./roms/tests/5-quirks.ch8".to_string(), true);
    }

}

fn run_emulator(path: String, test:bool) {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("Chip-8", "rust-sdl");

    let mut screen = match sdl::video::set_video_mode(64 * SIZE, 32 * SIZE, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let pixel_handler = BasicPixelHandler {
        screen: &mut screen
    };
    let keyboard_handler = BasicKeyboardHandler {
        status: [false; 16],
    };
    let random_handler = BasicRandomHandler {
        rng: rand::thread_rng()
    };

    let mut vm = chip8::vm::VM ::new(
        pixel_handler,
        keyboard_handler,
        random_handler
    );

    // read bytes from file
    let content = std::fs::read(path).unwrap();
    #[cfg(debug_assertions)]
    println!("content : {:?}", content);

    vm.setmemory(content);

    let mut last = SystemTime::now();

    let accepted = [
        Key::Num1, Key::Num2, Key::Num3, Key::Num4,
        Key::A,    Key::Z,    Key::E,    Key::R,
        Key::Q,    Key::S,    Key::D,    Key::F,
        Key::W,    Key::X,    Key::C,    Key::V,
    ];

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, is_pressed, _, _) => {
                    if k == Key::Escape {
                        break 'main;
                    }
                    if test && is_pressed && k == Key::Space {
                        thread::sleep(time::Duration::from_millis(500));
                        panic!("Test failed !");
                    }
                    if accepted.contains(&k) {
                        let nb = accepted.iter().position(|&x| x == k).unwrap();
                        vm.keyboardhandler.status[nb] = is_pressed;
                    }
                }
                _ => {}
            }
        }
        // we do one tick
        let time = SystemTime::now().duration_since(last).unwrap();
        if time.as_millis() > 1000 / FPS {// 16/60, 60FPS
            last = SystemTime::now();
            vm.decrease_timer();
        }
        thread::sleep(Duration::new(0, 1_000_000_000 / FREQUENCY));
        vm.process();
        vm.pixelhandler.screen.flip();
        #[cfg(debug_assertions)]
        println!("{:?}", vm.keyboardhandler.status);
    }

    sdl::quit();
}

fn main() {
    // TODO : take a file path in argument and differenciate ch8/a8
    let args = Args::parse();

    run_emulator(args.file, false);
}
