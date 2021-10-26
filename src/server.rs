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
            listener.accept();
        }

    }
}
