use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

use tor_stream::TorStream;
use torzkp::{
    ask_directory_authority_to_verify_proof, ask_exit_node_for_proof, handle_client,
    init_directory_authority, init_exit_node,
};

const MIN_OPTION: i8 = 1;
const MAX_OPTION: i8 = 3;

const CLIENT: i8 = 1;
const EXIT_NODE: i8 = 2;
const DIRECTORY_AUTHORITY: i8 = 3;

fn main() {
    let mut input = String::new();

    loop {
        println!("What kind of node are you ?");
        println!("  1. Client");
        println!("  2. Exit node");
        println!("  3. Directory Authority");
        print!("> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the input");

        let selected_option = input.trim().parse::<i8>();

        match selected_option {
            Ok(value) if MIN_OPTION <= value && value <= MAX_OPTION => {
                break;
            }
            Ok(_) => {
                println!("Invalid option");
                print!("\n\n");
                input.clear();
            }
            Err(_) => {
                println!("Unexpected error");
                std::process::exit(0);
            }
        }
    }

    let selected_option = input.trim().parse::<i8>().unwrap();
    match selected_option {
        CLIENT => {
            let onion_direction = get_onion_direction_cli();
            let direction = format!("{}:1234", onion_direction);
            let stream = TorStream::connect(&direction[..]);
            if let Err(error) = &stream {
                println!("Failed to connect: {}", error);
                std::process::exit(0);
            }
            println!("Successfully connected to exit node in port 9050");
            let stream = stream.unwrap();
            let proof = ask_exit_node_for_proof(stream);
            if let Err(_) = proof {
                println!("Error asking for proof");
                std::process::exit(0);
            }
            println!("Proof received");

            let stream = TcpStream::connect("localhost:9051");
            if let Err(error) = &stream {
                println!("Failed to connect: {}", error);
                std::process::exit(0);
            }
            println!("Successfully connected to directory authority in port 9050");
            let stream = stream.unwrap();
            match ask_directory_authority_to_verify_proof(stream, &proof.unwrap()) {
                Ok(result) => {
                    println!("Proof verification worked = {}", result);
                }
                Err(_) => {
                    println!("Error while verifying proof");
                }
            }
        }
        EXIT_NODE => {
            init_exit_node();
            start_server(9050);
        }
        DIRECTORY_AUTHORITY => {
            init_directory_authority();
            start_server(9051);
        }
        _ => (),
    }
}


fn get_onion_direction_cli() -> String {
    let mut onion_direction = String::new();
    println!("Introduce the onion direction you want to connect to");
    print!("> ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut onion_direction)
        .expect("Could not read the input");

    String::from(onion_direction.trim())
}

fn start_server(port: u16) {
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on port 9050");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    drop(listener);
}
