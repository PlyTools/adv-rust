// Declare the 'build' module, which corresponds to the 
// 'build' directory
mod build {
    // Inside the 'build' module, declare the 'compile' and 
    // 'config' sub-modules
    pub mod compile;
    pub mod config;
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Use functions from the compile module
    build::compile::compile_protos()?;

    // Use functions from the config module
    build::config::configure_build();

    println!("Build script succeeded!");
    
    Ok(())
}
