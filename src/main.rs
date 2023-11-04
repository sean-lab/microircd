use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use log::{info, error};

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone failed..."));
    let mut writer = stream;

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection was closed
                    break;
                }

                // Echo back the received message for now
                if let Err(e) = writer.write_all(line.as_bytes()) {
                    error!("Failed to write to socket: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("Error reading from socket: {}", e);
                break;
            }
        }
    }

    info!("Client disconnected");
}

fn main() {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:6667").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New client connected");
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                error!("Failed to accept client: {}", e);
            }
        }
    }
}
