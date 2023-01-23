use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream}, 
    thread,
    str,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:42531").unwrap();

    println!("Listening on {:?}", listener.local_addr());

    let mut connection_id = 0;
    for stream in listener.incoming() {
        println!("{connection_id}: new connection");
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(connection_id, stream);
            println!("{connection_id}: complete");
        });

        connection_id += 1;
    }
}

fn handle_connection(id: i32, mut stream: TcpStream) {
    //let mut buf_reader = BufReader::new(&stream);
    //let mut buf_writer = BufWriter::new(&stream);

    //let mut line = String::new();

    //buf_reader.read_line(&mut line).unwrap();
    //let mut request_parts = line.trim().split(" ");

    //println!("Start request {:?}", line);

    // let (method, path, version) = (
    //     request_parts.next().unwrap(),
    //     request_parts.next().unwrap(),
    //     request_parts.next().unwrap(),
    // );

    // let mut content_length: Option<usize> = if method == "GET" { Some(0) } else { None };

    // let mut headers = Vec::new();
    // while buf_reader.read_line(&mut line).is_ok() && !line.trim().is_empty() {
    //     if line.starts_with("Content-Length:") {
    //         let length = line.split(":").skip(1).next().unwrap().trim();
    //         content_length = Some(str::parse::<usize>(length).unwrap());
    //     }

    //     headers.push(line);

    //     line = String::new();
    // }

    // buf_writer.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    // match content_length {
    //     Some(content_length) => {
    //         buf_writer
    //             .write_all(format!("Content-Length: {content_length}\r\n").as_bytes())
    //             .unwrap();
    //     }
    //     _ => {}
    // }
    // buf_writer.write_all(b"\r\n").unwrap();
    // buf_writer.flush().unwrap();

    let mut buf = [0; 1024];
    let mut total: usize = 0;
    // while content_length.is_none() || content_length.unwrap() > total {
    loop {
        let size = stream.read(&mut buf).unwrap();
        println!("{id}: Read {size}");

        if size == 0 {
            stream.flush().unwrap();
            break;
        }

        println!("{id}: Writing {:?}", str::from_utf8(&buf[..size]).unwrap());

        stream.write_all(&buf[..size]).unwrap();
        
        total += size;
    }
}
