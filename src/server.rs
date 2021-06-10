
pub struct Server {
    addr: String, 
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    pub fn run(self) {
        println!("HTTP Server listening: {}", self.addr)
    }
}
