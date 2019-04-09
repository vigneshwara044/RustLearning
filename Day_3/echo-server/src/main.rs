extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;


fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listner = TcpListener::bind(&addr).unwrap();

    let server = listner.incoming().for_each(|socket| {

        let (reader, writer) = socket.split();

        let amount = io::copy(reader, writer);

        let msg = amount.then( |result|{
            match result {
                Ok((amount,_, _)) => println!("wrote {} bytes", amount),
                Err(e)            => println!("error: {}", e),  
            }
            Ok(())    
        });

        tokio::spawn(msg); 
        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });
    println!("server running on localhost:6142");

    tokio::run(server);
}
