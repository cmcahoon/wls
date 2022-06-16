#![deny(warnings)]

extern crate core;

mod fs;
mod utils;

use std::env;
use std::process::exit;
use crate::fs::recurse;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ERROR: Only one path argument is supported
    if args.len() > 2 {
        println!("ERROR: There can only be a single argument.");
        exit(1);
    }

    // DEFAULT: Use current directory if no path is provided as an argument
    let mut path = ".";
    if args.len() == 2 {
        path = &args[1];
    }

    // Iterate the filesystem starting at the path
    match recurse(path) {
        Ok(_) => {}, // intentionally empty
        Err(err) => println!("ERROR: {}", err),
    };
}
