use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        println!("running!!! on {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(error) => {
                            println!("some error on read stream {}", error);
                        }
                    }
                }
                Err(_) => {
                    println!("error")
                }
            }

        }
    }
}
