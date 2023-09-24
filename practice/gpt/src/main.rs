use std::env::args;

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    let key = args().nth(1).unwrap();

    // Creating a new ChatGPT client.
    // Note that it requries an API key, and uses tokens
    // from your OpenAPI account balance.
    let client = ChatGPT::new(key)?;

    // Sending a pre-specified message and getting the completion
    let response = client
        .send_message("Tell me a joke about Rust language.")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
