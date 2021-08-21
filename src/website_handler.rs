use super::server::Handler;
use super::http::{Request, Response, StatusCode};
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::OK, Some("<H1>Incredible webpage!</H1>".to_string()))
    }
}