use hyper::{Server, Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use ring::signature::{self, EcdsaKeyPair, KeyPair, UnparsedPublicKey};
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on https://{}", addr);

    server.await.unwrap();
}

fn load_ecdsa_from_file(file_path: &str) -> Result<EcdsaKeyPair, Box<dyn Error>> {
    // Load ECDSA from the private key file
    let mut file = File::open(file_path).expect("private key file not found");
    let mut key_bytes = Vec::new();
    file.read_to_end(&mut key_bytes)
        .expect("failed to read private key file");

    let key_pair = EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        key_bytes.as_slice(),
    ).expect("failed to load key pair");

    Ok(key_pair)
}


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Request: {:?}", req);

    let key_file_path = "./private_key.pem";
    let key_pair = load_ecdsa_from_file(key_file_path).unwrap();
    let public_key = key_pair.public_key();
    let public_key = UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_FIXED, public_key);

    // Verify the signature in the request header
    let signature_header = req.headers().get("X-Signature").unwrap().as_bytes();
    let signature = base64::decode(signature_header).unwrap();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    // verify the signature by using key_pair
    let is_verified = public_key.verify(&body_bytes, &signature).is_ok();
    println!("Signature verification result: {:?}", is_verified);

    Ok(Response::new("Hello".into()))
}

