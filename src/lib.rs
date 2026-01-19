use std::{
    fs,
    io::{self, BufReader, prelude::*, stdin},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

pub fn client() {
    let mut t = TcpStream::connect("127.0.0.1:7878").unwrap();

    t.write("Hello, Server".as_bytes()).unwrap();
}

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // server thread
    //thread::spawn(|| {
    //    handle_server();
    //});
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            let mut reader = BufReader::new(&stream);
            let mut buffer = String::new();

            match reader.read_to_string(&mut buffer) {
                Ok(_) => println!("Received: {}", buffer),
                Err(e) => println!("Read Error: {}", e),
            }
        });
    }
}
