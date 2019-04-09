extern crate tokio;
extern crate futures;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;


fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listner = TcpListener::bind(&addr).unwrap();

    let server = li
}
