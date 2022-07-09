use autoincrement::prelude::*;
use oxide::ThreadPool;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

mod commands;
mod deserializer;
pub mod handler;
mod pg;
mod serializer;
pub mod wire;

#[derive(AsyncIncremental, PartialEq, Eq, Debug)]
struct RequestId(u32);

fn main() {
    dotenv::dotenv().ok();
    let listener = TcpListener::bind("127.0.0.1:37017").unwrap();
    let pool = ThreadPool::new(10);
    let generator = RequestId::init();

    println!("Server listening on port 37017...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let id = generator.pull();

        stream.set_nodelay(true).unwrap();

        pool.execute(|| {
            handle_connection(stream, id);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, id: RequestId) {
    let mut buffer = [0; 1204];
    let addr = stream.peer_addr().unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(read) => {
                if read < 1 {
                    stream.flush().unwrap();
                    break;
                }

                use std::time::Instant;
                let now = Instant::now();

                let op_code = wire::parse(&buffer).unwrap();
                println!("Request ({}b): {:?}\t{}", read, op_code, addr);

                let mut response = handler::handle(id.0, op_code).unwrap();
                response.flush().unwrap();
                // println!("*** Hex Dump:\n {}", pretty_hex(&response));

                let elapsed = now.elapsed();
                println!("Processed {}b in {:.2?}\n", response.len(), elapsed);
                stream.write_all(&response).unwrap();
            }
            Err(e) => {
                println!("[{}-connection] Error: {}", id.0, e);
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
        };
    }
}
