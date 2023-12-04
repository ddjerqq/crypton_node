use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown, ToSocketAddrs};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct Server {
    listener: TcpListener,
    handles: Vec<JoinHandle<()>>,
    streams: Vec<Arc<Mutex<TcpStream>>>,
}

impl Server {
    fn handle_client(stream_lock: Arc<Mutex<TcpStream>>) {
        let mut buffer = Vec::new();

        loop {
            if let Ok(mut stream) = stream_lock.lock() {
                if let Err(e) = stream.read_to_end(&mut buffer) {
                    println!("Error reading from stream: {}", e);
                    let _ = stream.shutdown(Shutdown::Both);
                    break;
                }

                println!("{:#?}", buffer);

                // we will eventually have different logic here.
                if let Err(e) = stream.write_all(&buffer) {
                    println!("Error writing to stream: {}", e);
                    let _ = stream.shutdown(Shutdown::Both);
                    break;
                }
            }
        }

    }

    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        Ok(Self {
            listener,
            handles: Vec::new(),
            streams: Vec::new(),
        })
    }

    pub fn listen(mut self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let stream = Arc::new(Mutex::new(stream));

                self.streams.push(stream.clone());

                let handle = thread::spawn(move || {
                    Server::handle_client(stream)
                });

                self.handles.push(handle);
            }
        }

        Ok(())
    }
}

// impl Drop for Server {
//     fn drop(&mut self) {
//         for stream_lock in self.streams.iter() {
//             if let Ok(stream) = stream_lock.lock() {
//                 let _ = stream.shutdown(Shutdown::Both);
//             }
//         }
//
//         for handle in self.handles.drain(..) {
//             let _ = handle.join();
//         }
//     }
// }