// Copyright (c) 2023. 
// All rights reserved by Liam Ren
// This code is licensed under the MIT license.
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use hyper::{Server, Body, Request, Response, header, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use ring::signature::{self, EcdsaKeyPair, KeyPair, UnparsedPublicKey};
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;


fn load_ecdsa_from_file(file_path: &str) -> Result<EcdsaKeyPair, Box<dyn Error>> {
    // Load ECDSA from the private key file
    let mut file = File::open(file_path)?;
    let mut key_bytes = Vec::new();
    file.read_to_end(&mut key_bytes)?;
    let key_pair = EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        key_bytes.as_slice(),
    )?;
    Ok(key_pair)
}

async fn handle_request(req: Request<Body>, public_key: Arc<Mutex<UnparsedPublicKey<<EcdsaKeyPair as KeyPair>::PublicKey>>>) -> Result<Response<Body>, hyper::Error> {
    println!("Request: {:?}", req);

    // Verify the signature in the request header
    if let Some(signature_header) = req.headers().get("X-Signature") {
        let signature = base64::decode(signature_header.as_bytes()).unwrap_or_else(|_| Vec::new());
        let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
        // verify the signature by using key_pair
        let is_verified = public_key.lock().await.verify(&body_bytes, &signature).is_ok();
        
        let msg = if is_verified {
            "Signature verified successfully."
        } else {
            "Signature verification failed."
        };
        Ok(Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .body(msg.into())
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "text/plain")
            .body("Missing X-Signature header.".into())
            .unwrap())
    }

}

#[tokio::main]
async fn main() {
    let key_file_path = "./private_key.pem";
    let key_pair = load_ecdsa_from_file(key_file_path).unwrap();
    let public_key_bytes = key_pair.public_key().clone();
    let public_key = Arc::new(Mutex::new(
        UnparsedPublicKey::new(
            &signature::ECDSA_P256_SHA256_FIXED, 
            public_key_bytes
        )
    ));

    let make_svc = make_service_fn(move |_conn| {
        let public_key = public_key.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, public_key.clone()))) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on https://{}", addr);

    server.await.unwrap();
}
