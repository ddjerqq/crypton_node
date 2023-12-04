use std::env::args;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

mod transaction;
mod wallet;
mod crypto;
mod block;
mod parallel_miner;
mod util;
mod miner;
mod peer;

fn server() {
    let server = peer::Server::bind("localhost:3333").unwrap();
    println!("Server listening on port 3333");
    server.listen().unwrap();
}

fn client() {
    let mut client = peer::Client::connect("localhost:3333").unwrap();
    println!("Client connected to port 3333");

    loop {
        println!("sending hello");
        client.write(b"hello").unwrap();

        let buffer = client.read().unwrap();
        println!("receiving from server: {:?}", from_utf8(&buffer));

        println!("sleeping");
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mut args = args();
    let is_server = args.any(|arg| arg == "-server");

    if is_server {
        server();
    } else {
        client();
    }
}