use std::net::TcpListener;
use std::io::Read;

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
                        },
                        Err(error) => println!("Read failed: {}", error),
                    }
                },
                Err(error) => println!("Connection failed: {}", error),
            }
        }
    }
}