use server::Server;
use http::request::Request;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
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
}

mod http 
{
    pub mod request {
        use super::method::Method;

        pub struct Request {
            method: Method,
            path: String,
            query: Option<String>,
            protocol: String,
        }
    }

    mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT, 
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
        }
    }
}