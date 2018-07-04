extern crate server;

use server::Server;

fn main() {
    let server = Server::new();
    server.run();
}
