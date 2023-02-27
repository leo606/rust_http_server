fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..14];

    dbg!(&string);
    dbg!(string_slice);

    let server = Server::new(string);
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

    fn run(self) {
        println!("running!!!")
    }
}
