#![no_std]
#![no_main]
#![allow(unused)]

extern crate rust_profanos;

use alloc::borrow::ToOwned;
use alloc::format;
use rust_profanos::libs::std;
use rust_profanos::libs::std::fs::File;
use rust_profanos::libs::std::io::Read;
use rust_profanos::libs as libs;
use rust_profanos::println;
use rust_profanos::utilities as utilities;

use libs::vesa;

extern crate alloc;

use alloc::boxed::Box;

use alloc::{vec, vec::Vec};

pub mod panichandler;

pub mod chip8;

use chip8::vm::{
    KeyboardHandler,
    PixelHandler, RandomHandler,
};

struct BasicPixelHandler{
    
}

impl PixelHandler for BasicPixelHandler {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let color = if on { 0xFFFFFF } else { 0x000000 };

        // Prepare vectors to hold the coordinates and the color
        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();
        let mut colors = Vec::new();

        // Loop through a 10x10 square starting from (x, y)
        for i in 0..10 {
            for j in 0..10 {
                x_coords.push((x*10 + i) as u32); // X coordinates shifted by i
                y_coords.push((y*10 + j) as u32); // Y coordinates shifted by j
                colors.push(color);            // Color for each pixel
            }
        }

        // Call set_pixels to set all the pixels in the 10x10 square
        vesa::set_pixels(x_coords, y_coords, colors);
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
    
}

impl RandomHandler for BasicRandomHandler {
    fn random(&mut self) -> u8 {
        // TODO
        return 0;
    }
}

static SIZE: isize = 10;
static FPS: u128 = 60;
static FREQUENCY: u32 = 500;

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello from Rust and Chip-8 !");

    let pixel_handler = BasicPixelHandler {};
    let keyboard_handler = BasicKeyboardHandler {
        status: [false; 16],
    };
    let random_handler = BasicRandomHandler {};

    let mut vm = chip8::vm::VM ::new(
        pixel_handler,
        keyboard_handler,
        random_handler
    );

    let args = std::env::args().collect();

    println!("{}", &args[1]);

    let mut file = File::open(&args[1]).unwrap();

    let mut content: Vec<u8> = vec![0; file.metadata().unwrap().len()];

    file.read(&mut content);

    vm.setmemory(content);

    loop {
        vm.process();
    }

    println!("Lets exit now !");
}