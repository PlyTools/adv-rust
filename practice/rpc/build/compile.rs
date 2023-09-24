pub fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    // Your compilation logic here
    println!("Compiling protos...");
    tonic_build::compile_protos("proto/user_service.proto")?;
    Ok(())
}
