use std::collections::HashMap;
use std::{io, thread};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;
use crate::protocol::message::Message;

type Clients = Arc<Mutex<HashMap<Uuid, TcpStream>>>;

pub struct Peer {
    pub id: Uuid,
    clients: Clients,
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
            id: Uuid::new_v4(),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn _t_broadcast(recv: Receiver<Message>, clients: Clients) {
        loop {
            let res = recv.recv();
            let msg = res.unwrap();

            let mut clients = clients.lock().unwrap();

            for (id, client) in clients.iter_mut() {
                match crate::protocol::send(client, &msg) {
                    Ok(_) => println!("sent {:?} to {:}", msg, id),
                    Err(e) => println!("error sending {:?} to {:}: {:?}", msg, id, e),
                }
            }
        }
    }

    fn _handle_stream(id: Uuid, stream: TcpStream, send: Sender<Message>) -> thread::JoinHandle<()> {
        let mut reader = io::BufReader::new(stream);

        thread::spawn(move || {
            loop {
                if let Ok::<Message, io::Error>(msg) = crate::protocol::recv(&mut reader) {
                    println!("received message from {}: {:?}", id, msg);
                    send.send(msg).expect("could not send message to broadcast thread");
                } else {
                    break;
                }
            }

            eprintln!("client {} disconnected", id);
            reader.into_inner()
                .shutdown(std::net::Shutdown::Both)
                .expect("could not shutdown stream");
        })
    }

    fn _t_listener<T: ToSocketAddrs>(addr: T, send: Sender<Message>, clients: Clients) {
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            let id = Uuid::new_v4();
            let stream = stream.expect("TODO fix");
            let writer = stream.try_clone().expect("TODO fix");

            clients.lock().unwrap().insert(id.clone(), writer);

            Peer::_handle_stream(id.clone(), stream, send.clone());
        }
    }

    pub fn run<T: 'static + ToSocketAddrs + Send + Sync>(
        &self,
        addr: T
    ) -> io::Result<(thread::JoinHandle<()>, thread::JoinHandle<()>)> {
        let (send, recv) = std::sync::mpsc::channel::<Message>();

        let listener_clients = self.clients.clone();
        let listen_handle = thread::spawn(move || {
            Peer::_t_listener(addr, send, listener_clients)
        });

        let broadcast_clients = self.clients.clone();
        let broadcast_handle = thread::spawn(move || {
            Peer::_t_broadcast(recv, broadcast_clients)
        });

        Ok((listen_handle, broadcast_handle))
    }
}