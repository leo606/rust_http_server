fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run()
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Server {
            address
        }
    }

    fn(self) {

    }
}
