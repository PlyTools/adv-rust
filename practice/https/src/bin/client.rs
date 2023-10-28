use hyper::{Body, Client, Request};
use ring::{rand, signature};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};


fn create_ecdsa_to_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a ECDSA key pair and save the private code to the privided file_path
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

fn load_ecdsa_from_file(file_path: &str) -> Result<signature::EcdsaKeyPair, Box<dyn Error>> {
    // Load ECDSA from the private key file
    let mut file = File::open(file_path).expect("private key file not found");
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

    let key_pair = load_ecdsa_from_file(key_file_path).unwrap();
    println!("{:?}", key_pair);

    // Create a request
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/api")
        .body(Body::from("Test for identable HTTPS request!"))
        .unwrap();

    // Sign the request
    let rng = rand::SystemRandom::new();
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
