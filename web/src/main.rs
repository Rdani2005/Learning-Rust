use web::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6543").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println!("Shutting down!");
}

fn handle_conn(mut stream : TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    let (status, file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "sleeping.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file).unwrap();

    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
