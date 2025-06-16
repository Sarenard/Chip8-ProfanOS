#![no_std]
#![no_main]
#![allow(unused)]

extern crate rust_profanos;

use rust_profanos::libs as libs;
use rust_profanos::println;
use rust_profanos::utilities as utilities;

extern crate alloc;

use alloc::boxed::Box;

use alloc::{vec, vec::Vec};

pub mod entry;
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
        // TODO
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

fn main() {
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

    let content: Vec<u8> = vec![];

    vm.setmemory(content);

    loop {
        vm.process();
    }

    println!("Lets exit now !");
}