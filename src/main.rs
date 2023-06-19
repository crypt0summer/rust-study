fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());//string literal vs string
    server.run();
}

struct Server {
    addr: String,
}

impl Server{
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */

struct Request {
    path: String,
    query_string: Option<String>, //Rust doesn't support "No value" - use Option enum
    method: Method,
}

enum Method { //Enums
    GET, 
    POST, 
    PUT, 
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

