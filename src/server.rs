use std::net::TcpListener;

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
      Server {
          address
      }
  }

  pub fn run(self) {
      let listener = TcpListener::bind(&self.address).unwrap();

      println!("running!!! on {}", self.address);
  }
}
