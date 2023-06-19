use std::net::TcpListener;

//Everything in Modules is private by default
//Every file in Rust is a module

pub struct Server {
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        //unwrap: if true, return tcplistener, if false, panic and log error
        let listener = TcpListener::bind(&self.addr).unwrap();//Recoverable error vs Unrecoverable error
    }
}

// OLD VERSION
// mod server{ 
//     pub struct Server {
//         addr: String,
//     }

//     impl Server{
//         pub fn new(addr: String) -> Self {
//             Self {
//                 addr
//             }
//         }

//         pub fn run(self) {
//             println!("Listening on {}", self.addr);
//         }
//     }
// }