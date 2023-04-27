#![allow(dead_code)]

// TODO: handle request and response headers
// TODO: implements mutiple threads
    // Module std::thread
    // Module std::sync

use server::Server;
use website_handler::WebsiteHandler;
use std::{env};

mod http;
mod server;
pub mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // reads de variables at compiletime
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); // reads the variables at runtime
    println!("public_path => {}", public_path);
    server.run(WebsiteHandler::new(public_path))
}
