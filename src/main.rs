#[allow(dead_code)]
use std::env::args;
use std::{io, thread};
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::protocol::Message;
use crate::protocol::peer::Peer;

mod transaction;
mod wallet;
mod crypto;
mod block;
mod parallel_miner;
mod util;
mod miner;
mod protocol;

const ADDR: &'static str = "127.0.0.1:1111";

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn client() {
    let r_stream = TcpStream::connect(ADDR).unwrap();
    let mut w_stream = r_stream.try_clone().unwrap();
    let mut reader = io::BufReader::new(r_stream);

    // listener thread
    thread::spawn(move || {
        loop {
            let msg: Message = protocol::recv(&mut reader).unwrap();
            println!("message received: {msg:?}");
        }
    });

    loop {
        let input = get_input();
        if input.contains("exit") {
            break;
        }

        let msg = Message::Text(input);
        protocol::send(&mut w_stream, &msg).unwrap();
    }
}

fn main() {
    let mut args = args();
    let is_server = args.any(|arg| arg == "-server");

    if is_server {
        let peer = Peer::new();
        let (l, b) = peer.run(ADDR).unwrap();

        l.join().unwrap();
        b.join().unwrap();

        return;
    }

    client();
}