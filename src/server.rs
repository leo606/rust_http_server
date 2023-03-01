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
      println!("running!!!")
  }
}
