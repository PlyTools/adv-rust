use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use serde_derive::Deserialize;
use std::error::Error;


#[derive(Deserialize, Debug)]
struct Post {
    // The field name must be alighed with the HTTPS Get Response
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

async fn https_get() -> Result<(), Box<dyn Error>> {
    let response: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("{:?}", response);
    Ok(())
}

fn create_wallet() -> Result<(), Box<dyn Error>> {
    // Generate a random wallet
    let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
    let index = 0u32;
    let password = "TREZOR123";

    // Access mnemonic phrase with password
    // Child key at derivation path: m/44'/60'/0'/0/{index}
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .index(index)?
        // Use this if your mnemonic is encrypted
        .password(password)
        .build()?;

    eprintln!("Wallet: {wallet:?}");

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    https_get().await?;    
    let _ = create_wallet();
    Ok(())
}
