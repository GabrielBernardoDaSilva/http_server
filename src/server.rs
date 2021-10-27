use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}
// type int = i32;
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        // let a :int = 10;
        let listener = TcpListener::bind(&self.addr).unwrap();
        //label loops 'outer loop{}
        loop {
            let res = listener.accept();
            match res {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    
                                },
                                Err(e) => {
                                    println!("failed to parse the {:?}", e);
                                }
                            
                            }
                            
                        }
                        Err(e) => {
                            println!("Failed to establish connection {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
