extern crate tokio;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(&self) {
        let addr = "0.0.0.0:50000".parse().unwrap();
        let listener = TcpListener::bind(&addr)
            .expect("unable to bind TCP listener");

        let server = listener.incoming()
            .map_err(|e| eprintln!("accept failed = {:?}", e))
            .for_each(|sock| {
                let (reader, writer) = sock.split();
                let bytes_copied = copy(reader, writer);

                let handle_conn = bytes_copied.map(|amt| {
                    println!("wrote {:?} bytes", amt)
                }).map_err(|err| {
                    eprintln!("IO error {:?}", err)
                });

                tokio::spawn(handle_conn)
            });

        println!("Server running on {}", addr);
        tokio::run(server);
    }
}