use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        // syntax sugar for infinate loop
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {}
                Err(msg) => println!("Connection failed: {}", msg),
            }
        }
    }
}
