use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
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

        //unwrap: if true, return tcplistener, if false, panic and log error (unrecoverable error)
        let listener = TcpListener::bind(&self.addr).unwrap();//Recoverable error vs Unrecoverable error
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    //read bytecode from stream
                    match stream.read(&mut buffer){
                        Ok(_)=>{
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                           
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                            // let res : &Result<Request, String> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
          
        }
        // 'outer: loop {//label
        //     loop {
        //         continue 'outer;
        //     }
        // }

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