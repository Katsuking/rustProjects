use crate::http::{Response, StatusCode};

use super::server::Handler;
pub struct WebsiteHandler {}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Hello world</h1>".to_string()))
    }
}
