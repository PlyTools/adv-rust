use hyper::{Body, Client, Request};
use ring::{rand, signature};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn create_ecdsa_to_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a ECDSA key pair and save the private code to private_key.pem
    let rng = rand::SystemRandom::new();
    let private_key =
        signature::EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .expect("failed to generate key pair");

    // Write private key to file
    File::create(file_path)
        .expect("private key file not found")
        .write_all(private_key.as_ref())
        .expect("failed to write private key file");

    Ok(())
}

// Load ECDSA from the private key file private_key.pem and return the public key
fn load_ecdsa_from_file() -> Result<signature::EcdsaKeyPair, Box<dyn Error>> {
    let mut file = File::open("./private_key.pem").expect("private key file not found");
    let mut key_bytes = Vec::new();
    file.read_to_end(&mut key_bytes)
        .expect("failed to read private key file");

    let key_pair = signature::EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        key_bytes.as_slice(),
    ).expect("failed to load key pair");

    Ok(key_pair)
}

#[tokio::main]
async fn main() {
    let key_file_path = "./private_key.pem";
    create_ecdsa_to_file(key_file_path).unwrap();

    let key_pair = load_ecdsa_from_file().unwrap();
    println!("{:?}", key_pair);

    // Create a request
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .body(Body::from("Test for identable HTTPS request!"))
        .unwrap();

    // Sign the request
    let rng = rand::SystemRandom::new();
    let signature = key_pair.sign(&rng, req.body().as_ref()).unwrap();

    // Create a new request with the signature header
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .header("X-Signature", base64::encode(&signature))
        .body(Body::from("Test for identable HTTPS request!"))
        .unwrap();


    let client = Client::new();
    let resp = client.request(req).await.unwrap();
    println!("{:?}", resp);
}
