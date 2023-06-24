//! Copyright (c) Liam 2023
//! All rights reserved. Licensed under MIT.
//!
//! The examples about handling recoverable errors in Chapter 9.2

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
