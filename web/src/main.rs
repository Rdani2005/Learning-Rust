use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*,BufReader},
    thread,
    time::Duration,
};

use web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6543").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        let pool = ThreadPool::new(4);


        pool.execute(|| {
            handle_conn(stream);
        });
    }

}

fn handle_conn(mut stream : TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleeping.html")
        }
        
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();
    
    let response = 
        format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
    
    stream.write_all(response.as_bytes()).unwrap();
    println!("New connection!");
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();


}
