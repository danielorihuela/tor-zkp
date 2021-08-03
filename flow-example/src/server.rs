use crate::messages::{decode_message, encode_message, MessageType};
use crate::zpie::{generate_proof, init_prover, init_verifier, verify_proof};

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn init_directory_authority() {
    unsafe {
        init_verifier();
    };
}

pub fn init_onion_service() {
    unsafe {
        init_prover();
    };
}

pub fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 1024];

    let length = stream.read(&mut data);
    if let Err(_) = length {
        send_error_message(stream, "An error occurred");
        return;
    }

    let length = length.unwrap();
    let response = handle_request(&data[0..length]);
    if let None = response {
        send_error_message(stream, "Incorrect message type");
        return;
    }

    let response = response.unwrap();
    println!("Response = {}", response);
    println!("Sending response to client...");
    stream.write_all(response.as_bytes()).unwrap();
}

fn send_error_message(mut stream: TcpStream, message: &str) {
    println!(
        "Incorrect message type. Terminating connection with {}",
        stream.peer_addr().unwrap()
    );

    let response = format!("{}. Terminating connection.", message);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_request(request: &[u8]) -> Option<String> {
    let decoded_request = String::from_utf8_lossy(request);
    let (message_type, data) = decode_message(&decoded_request);

    match message_type {
        // Message for onion service
        Some(MessageType::NeedProof) => {
            println!("Client needs a proof. Creating proof...");
            unsafe {
                init_prover();
                generate_proof();
            }

            let proof = fs::read_to_string("data/proof.params").unwrap();
            let response = encode_message(&MessageType::GeneratedProof, &proof);
            Some(response)
        }
        // Message for directory authority
        Some(MessageType::VerifyProof) => {
            println!("Client needs to verify a proof. Verifying proof...");
            fs::write("data/proof.params", data).unwrap();

            let verification_was_successful: u8;
            unsafe {
                verification_was_successful = verify_proof();
            };

            println!(
                "Verification was successful = {}",
                &verification_was_successful
            );
            let message_type = if verification_was_successful == 1 {
                MessageType::VerifiedProof
            } else {
                MessageType::IncorrectProof
            };
            fs::remove_file("data/proof.params").unwrap();

            let response = encode_message(&message_type, "");
            Some(response)
        }
        // Other types of messages are not meant for servers
        _ => None,
    }
}
