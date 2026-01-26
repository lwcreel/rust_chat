use rust_chat::{client, server};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    println!("{}", mode);

    if mode == "server" {
        server();
    } else {
        client();
    }
}
