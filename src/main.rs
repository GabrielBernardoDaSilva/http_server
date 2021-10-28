#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]


mod http;
mod server;
mod website_handler;

use server::Server;
use http::Request;
use http::Method;
use std::env;
use std::fmt::format;
use website_handler::WebsiteHandler;



    


fn main() {

    let deafault_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(deafault_path);
    println!("{}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
