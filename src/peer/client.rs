use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::io::{Read, Write};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect<T: ToSocketAddrs>(addr: T) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Ok(Self { stream })
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        for chunk in data.chunks(64) {
            self.stream.write(chunk)?;
        }

        Ok(())
    }

    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        let mut message = Vec::new();

        'outer: loop {
            let mut buffer = [0u8; 64];

            'inner: loop {
                match self.stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("size: {}", size);

                        if size == 0 {
                            break 'inner;
                        }

                        message.extend_from_slice(&buffer[..size]);
                    }

                    Err(e) => {
                        println!("Error reading from stream: {}", e);
                        let _ = self.stream.shutdown(Shutdown::Both);
                        break 'outer;
                    }
                }
            }
        }

        return Ok(message);
    }
}