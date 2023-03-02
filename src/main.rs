// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("accepted new connection");
                    let mut buf = [0; 128];
                    loop {
                        let buf_read = stream.read(&mut buf);
                        println!("read {:?} bytes", buf_read);
                        if let Ok(0) = buf_read {
                            println!("read {:?}", String::from_utf8(buf.to_vec()));
                            break;
                        }
                        println!("stream write call");
                        stream.write(b"+PONG\r\n").unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
