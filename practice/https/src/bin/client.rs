use anyhow::{Context, Result};
use base64;
use hyper::{Body, Client, Request};
use ring::{
    rand::SystemRandom,
    signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING},
};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

fn create_ecdsa_to_file(file_path: &str) -> Result<()> {
    // Create a ECDSA key pair and save the private code to the privided file_path
    let rng = SystemRandom::new();
    let private_key = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .context("Failed to generate ECDSA key pair")?;

    // Write private key to file
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .context("Failed to open file for writing")?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(private_key.as_ref())
        .context("Failed to write private key to file")?;

    Ok(())
}

fn load_ecdsa_from_file(file_path: &str) -> Result<EcdsaKeyPair> {
    // Load ECDSA from the private key file
    let file = File::open(file_path).context("Failed to open file for reading")?;
    let mut reader = BufReader::new(file);
    let mut key_bytes = Vec::new();
    reader
        .read_to_end(&mut key_bytes)
        .context("Failed to read private key from file")?;

    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, key_bytes.as_slice())
        .context("Failed to create ECDSA key pair from file")?;

    Ok(key_pair)
}

#[tokio::main]
async fn main() {
    let key_file_path = "./private_key.pem";
    create_ecdsa_to_file(key_file_path).unwrap();

    let key_pair = load_ecdsa_from_file(key_file_path).unwrap();
    println!("{:?}", key_pair);

    // Create a request
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .body(Body::from("Test for identable HTTPS request!"))
        .unwrap();

    // Sign the request
    let rng = SystemRandom::new();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let signature = key_pair.sign(&rng, body_bytes.as_ref()).unwrap();

    // Create a new request with the signature header
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .header("X-Signature", base64::encode(signature))
        .body(Body::from("Test for identable HTTPS request!"))
        .unwrap();

    let client = Client::new();
    let resp = client.request(req).await.unwrap();
    println!("{:?}", resp);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_load_ecdsa() {
        let key_file_path = "./private_key.pem";
        create_ecdsa_to_file(key_file_path).unwrap();

        let key_file_path = "./private_key.pem";
        let key_pair = load_ecdsa_from_file(key_file_path).unwrap();
        println!("{:?}", key_pair);
    }
}
