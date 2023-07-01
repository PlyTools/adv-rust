//! Copyright (c) Liam 2023
//! All rights reserved. License under MIT
//! 
//! This is the I/O project in the Chapter 12


use std::env;
use std::process;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
