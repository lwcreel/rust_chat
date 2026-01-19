use rust_chat::{client, server};
use std::{
    env, fs,
    io::{self, BufReader, prelude::*, stdin},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    if mode == "server" {
        server();
    } else {
        client();
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
