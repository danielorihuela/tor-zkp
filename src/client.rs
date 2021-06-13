use crate::messages::{decode_message, encode_message, MessageType};

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

use tor_stream::TorStream;

pub fn ask_exit_node_for_proof(mut stream: TorStream) -> Result<String, Box<dyn Error>> {
    let message = encode_message(&MessageType::NeedProof, "");
    stream.write_all(message.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let (_, data) = decode_message(&response);

    Ok(String::from(data))
}

pub fn ask_directory_authority_to_verify_proof(
    mut stream: TcpStream,
    proof: &str,
) -> Result<bool, Box<dyn Error>> {
    let message = encode_message(&MessageType::VerifyProof, proof);
    stream.write_all(message.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let (message_type, _) = decode_message(&response);

    match message_type {
        Some(MessageType::VerifiedProof) => return Ok(true),
        Some(MessageType::IncorrectProof) => return Ok(false),
        Some(_) | None => return Ok(false),
    }
}
