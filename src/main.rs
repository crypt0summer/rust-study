use server::Server;
use http::Request;
use http::Method; //same as http::method::Method;

mod server; //Relative path
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());//string literal vs string
    server.run();
}




