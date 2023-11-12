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

use std::str::FromStr;

use anyhow::{Context, Result};
use ethers::{
    core::types::{Signature, H160, H256},
    utils::keccak256,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};

fn recover_address_from_signature(message: &[u8], signature_str: &str) -> Result<H160> {
    // Hash the message with the Ethereum prefix
    let hashed_msg = keccak256(
        format!(
            "\x19Ethereum Signed Message:\n{}{:?}",
            message.len(),
            message
        )
    );

    // Convert the signature to its r, s, and v components
    let signature = Signature::from_str(signature_str).unwrap();

    // Recover the Ethereum address
    let address = signature.recover(H256::from_slice(&hashed_msg)).unwrap();

    Ok(address)
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>> {
    println!("Request: {:?}", req);

    // Verify the signature in the request header
    if let Some(signature_header) = req.headers().get("X-Signature") {
        let signature = String::from_utf8(signature_header.as_ref().to_vec())
            .context("Failed to parse signature header")?;
        println!("Signature: {}", signature);
        let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

        // Recover the Ethereum address from the signature
        let mut msg = String::new();
        match recover_address_from_signature(body_bytes.as_ref(), signature.as_str()) {
            Ok(address) => {
                msg.push_str(&format!("Signed by address: {}", address));
            }
            Err(e) => {
                msg.push_str(&format!("Failed to recover address from signature: {}", e));
            }
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
    let make_svc = make_service_fn(move |_conn| async move {
        Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req)))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on https://{}", addr);

    server.await.unwrap();
}
