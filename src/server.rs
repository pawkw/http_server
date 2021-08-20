use std::net::TcpListener;
use std::io::Read;
use crate::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}.", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(error) => println!("Parse request failed: {}", error),
                            }
                        },
                        Err(error) => println!("Read failed: {}", error),
                    }
                },
                Err(error) => println!("Connection failed: {}", error),
            }
        }
    }
}