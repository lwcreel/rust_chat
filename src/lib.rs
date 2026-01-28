use std::{
    io::{self, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
};

pub struct Server {
    listener: TcpListener,
    connections: Vec<Client>,
}

pub struct Client {
    username: String,
    connection: TcpStream,
}

pub fn client() {
    println!("Please Enter Your Username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Client Error");
    let username = username.trim_end();

    let mut connection = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        let mut message = String::from(format!("[{}]: ", username));
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Client Error");
        message.push_str(input.as_str());
        message.push_str("\n");

        connection
            .write(message.as_bytes())
            .expect("Error in Sending Client Message to Server");

        connection
            .flush()
            .expect("Error in Sending Client Message to Server")
    }
}

pub fn server() {
    let mut server = Server {
        listener: TcpListener::bind("127.0.0.1:7878").unwrap(),
        connections: Vec::new(),
    };

    // server thread
    thread::spawn(|| {
        handle_server();
    });

    for stream in server.listener.incoming() {
        server.connections.push(Client {
            connection: stream.unwrap(),
            username: String::from("temp"),
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

fn handle_client(stream: TcpStream) {
    println!("Hello, New Chatter!");

    loop {
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        match reader.read_line(&mut buffer) {
            Ok(_) => println!("{}", buffer),
            Err(e) => println!("Read Error: {}", e),
        }
    }
}

fn broadcast(message: &str) {
    println!("{}", message);
}
