use std::io;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

mod parser;

fn http_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("error occured while accepting connection: {}", e);
                continue;
            }
        };

        let _ = thread::spawn(move || -> io::Result<()> {
            loop {
                let mut b = [0; 1024];
                let n = stream.read(&mut b)?;
                if n > 0 {
                    stream.write(&b[0..n])?;
                } else {
                    break ();
                }
            }
            Ok(())
        });
    }
    Ok(())
}

fn main() {
    match http_server() {
        Ok(_) => (),
        Err(e) => println!("error occured: {:?}", e),
    }
}
