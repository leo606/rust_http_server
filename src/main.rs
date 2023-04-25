#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
pub mod website_handler;

fn main() {
    let string = String::from("127.0.0.1:8080");

    let server = Server::new(string);
    server.run(WebsiteHandler)
}
