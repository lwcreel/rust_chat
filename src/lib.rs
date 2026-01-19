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
    thread::spawn(|| {
        handle_server();
    });
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

fn handle_server() {
    println!("Waiting for Chatters...");
    loop {
        let mut message = String::from("[Server]: ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Server Error");
        message.push_str(String::as_str(&input));

        broadcast(String::as_str(&message));
    }
}

fn broadcast(message: &str) {
    print!("{}", message);
}
