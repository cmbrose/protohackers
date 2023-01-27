use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream}, 
    thread,
    str,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Listening on {:?}", listener.local_addr());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut buf_writer = BufWriter::new(&stream);

    let mut line = String::new();

    buf_reader.read_line(&mut line).unwrap();
    let mut request_parts = line.trim().split(" ");

    let (method, _, _) = (
        request_parts.next().unwrap(),
        request_parts.next().unwrap(),
        request_parts.next().unwrap(),
    );

    let mut content_length: Option<usize> = if method == "GET" { Some(0) } else { None };

    let mut headers = Vec::new();
    while buf_reader.read_line(&mut line).is_ok() && !line.trim().is_empty() {
        if line.starts_with("Content-Length:") {
            let length = line.split(":").skip(1).next().unwrap().trim();
            content_length = Some(str::parse::<usize>(length).unwrap());
        }

        headers.push(line);

        line = String::new();
    }

    buf_writer.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    match content_length {
        Some(content_length) => {
            buf_writer
                .write_all(format!("Content-Length: {content_length}\r\n").as_bytes())
                .unwrap();
        }
        _ => {}
    }
    buf_writer.write_all(b"\r\n").unwrap();
    buf_writer.flush().unwrap();

    let mut buf = [0; 1024];
    let mut total: usize = 0;
    while content_length.is_none() || content_length.unwrap() > total {
        let size = buf_reader.read(&mut buf).unwrap();
        if size == 0 {
            buf_writer.flush().unwrap();
            break;
        }

        buf_writer.write_all(&buf[..size]).unwrap();
        
        total += size;
    }
}
