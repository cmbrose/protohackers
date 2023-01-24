use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream}, 
    thread,
    str,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

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
    let mut buf = [0; 1024];
    let mut total: usize = 0;
    loop {
        let size = stream.read(&mut buf).unwrap();
        if size == 0 {
            stream.flush().unwrap();
            break;
        }

        stream.write_all(&buf[..size]).unwrap();
        
        total += size;
    }
}
