use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                loop {
                    let mut buf = [0 as u8; 1024];
                    stream.read(&mut buf).unwrap();
                    let str;
                    match String::from_utf8(buf.to_vec()) {
                        Ok(string) => str = string,
                        Err(e) => {
                            println!("Error occured {e}");
                            continue;
                        }
                    }

                    println!("{str}");

                    for _ in str.match_indices("PING") {
                        stream.write_all(b"+PONG\r\n").unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
