use std::env;
use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use tor_stream::TorStream;
use torzkp::{
    ask_directory_authority_to_verify_proof, ask_onion_service_for_proof, handle_client,
    init_directory_authority, init_onion_service,
};

const MIN_OPTION: i8 = 1;
const MAX_OPTION: i8 = 3;

const CLIENT: i8 = 1;
const ONION_SERVICE: i8 = 2;
const DIRECTORY_AUTHORITY: i8 = 3;

fn main() {
    let mut automatic = false;
    let mut selected_option = -1;
    if let Ok(_) = env::var("ONION_SERVICE") {
        automatic = true;
        selected_option = ONION_SERVICE;
    }
    if let Ok(_) = env::var("DIRECTORY_AUTHORITY") {
        automatic = true;
        selected_option = DIRECTORY_AUTHORITY;
    }

    if !automatic {
        selected_option = select_node_type_cli();
    }

    match selected_option {
        CLIENT => {
            let onion_direction = get_onion_direction_cli();
            let direction = format!("{}:1234", onion_direction);
            let stream = TorStream::connect(&direction[..]);
            if let Err(error) = &stream {
                println!("Failed to connect: {}", error);
                std::process::exit(0);
            }
            println!("Successfully connected to onion service in port 1234");
            let stream = stream.unwrap();
            let proof = ask_onion_service_for_proof(stream);
            if let Err(_) = proof {
                println!("Error asking for proof");
                std::process::exit(0);
            }
            println!("Proof received");

            let stream = TcpStream::connect("localhost:1234");
            if let Err(error) = &stream {
                println!("Failed to connect: {}", error);
                std::process::exit(0);
            }
            println!("Successfully connected to directory authority in port 1234");
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
        ONION_SERVICE => {
            init_onion_service();
            start_server(1234);
        }
        DIRECTORY_AUTHORITY => {
            init_directory_authority();
            start_server(1234);
        }
        _ => (),
    }
}

fn select_node_type_cli() -> i8 {
    let mut input = String::new();

    loop {
        println!("What kind of node are you ?");
        println!("  1. Client");
        println!("  2. Onion Service");
        println!("  3. Directory Authority");
        print!("> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the input");

        let selected_option = input.trim().parse::<i8>();

        match selected_option {
            Ok(value) if MIN_OPTION <= value && value <= MAX_OPTION => {
                return input.trim().parse::<i8>().unwrap();
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
    println!("Server listening on port {}", port);
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
