use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Error on decoding request: {}", e)
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(err) => println!("Error on accept: {}", err.to_string())
            }
        }
    }
}
