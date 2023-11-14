use anyhow::{Context, Result};
use hyper::{Body, Client, Request};
use ethers::{
    prelude::*,
    signers::{coins_bip39::English, MnemonicBuilder},
    utils::{to_checksum, keccak256},
};
use env_logger;

fn generate_ethereum_account() -> Result<(String, String, LocalWallet)> {
    // Generate a new Ethereum account
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
        .build()
        .context("Failed to generate Ethereum account")?;

    let address = wallet.address();

    Ok((phrase.to_string(), to_checksum(&address, None), wallet))
}

fn sign_message(wallet: &LocalWallet, message: &[u8]) -> Result<String> {
    // Hash the message
    let hashed_msg = keccak256(format!("\x19Ethereum Signed Message:\n{}{:?}", message.len(), message));

    // Sign the message
    let signature = wallet.sign_hash(H256::from_slice(&hashed_msg))
        .context("Failed to sign message")?;

    Ok(signature.to_string())
}


#[tokio::main]
async fn main() {
    env_logger::init();

    let message = "Test for identable HTTPS request!";
    
    // Create a request
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .body(Body::from(message))
        .unwrap();

    // Sign the request
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

    // Generate a new Ethereum account
    let (mnemonic_phrase, address, wallet) = generate_ethereum_account().unwrap();
    log::debug!("Mnemonic phrase: {}", mnemonic_phrase);
    log::debug!("Address: {}", address);

    let signature = sign_message(&wallet, body_bytes.as_ref()).unwrap();
    log::debug!("Signature: {}", signature);

    // Create a new request with the signature header
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .header("X-Signature", signature)
        .body(Body::from(message))
        .unwrap();

    // Send the request
    let client = Client::new();
    let resp = client.request(req).await.unwrap();
    log::info!("Response: {:?}", resp);
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    log::debug!("Response message: {:?}", String::from_utf8(body_bytes.as_ref().to_vec()).unwrap());
}


