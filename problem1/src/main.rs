use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream}, 
    thread,
    str::{self, FromStr}, ops::Add,
};

use json::{
    object,
};

use num_bigint::{BigInt};

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

fn handle_connection(connection_id: i32, mut stream: TcpStream) {
    let mut request_bldr: Vec<u8> = Vec::new();

    let mut buf = [0; 1024];
    loop {
        let size = stream.read(&mut buf).unwrap();
        if size == 0 {
            stream.flush().unwrap();
            break;
        }

        let mut i = 0;
        let mut last_break = 0;
        while i < size {
            if buf[i] == 10 {
                request_bldr.extend_from_slice(&buf[last_break..i]);
                last_break = i;

                println!("{connection_id}: request {:?}", str::from_utf8(&request_bldr).unwrap());

                let response = handle_request(request_bldr.to_owned());                
                stream.write(&response).unwrap();
                stream.write(&[10]).unwrap();

                println!("{connection_id}: response {:?}", str::from_utf8(&response).unwrap());
                
                request_bldr.clear();
            }

            i += 1;
        }

        if last_break+1 < size {
            request_bldr.extend_from_slice(&buf[last_break..]);
        }
    }
}

fn handle_request(request_content: Vec<u8>) -> Vec<u8> {
    let mut response = object!{};

    let maybe_request_str = str::from_utf8(&request_content);
    if maybe_request_str.is_err() {
        return json::stringify(response).into_bytes();
    }

    let request_str = maybe_request_str.unwrap();

    let maybe_request = json::parse(request_str);
    if maybe_request.is_err() {
        return json::stringify(response).into_bytes();
    }

    let request = maybe_request.unwrap();

    if request["method"] != "isPrime" {
        return json::stringify(response).into_bytes();
    }

    response["method"] = "isPrime".into();

    // Verify that there is a value and that it's not a string (string will trip up the BigInt check)
    let maybe_prime = &request["number"];
    if maybe_prime.is_null() || maybe_prime.is_string() {
        return json::stringify(response).into_bytes();
    }

    // Check for an i64 which is a lot faster to check for prime
    let maybe_small_prime = maybe_prime.as_i64();
    if maybe_small_prime.is_some() {
        response["prime"] = is_small_prime(maybe_small_prime.unwrap()).into();
        return json::stringify(response).into_bytes();
    }

    // If it's too big to be an i64, try BigInt
    let big_prime_str = maybe_prime.to_string();

    let maybe_big_prime = BigInt::from_str(big_prime_str.as_str());
    if maybe_big_prime.is_ok() {
        response["prime"] = is_big_prime(maybe_big_prime.unwrap()).into();
        return json::stringify(response).into_bytes();
    }

    // Finally, it could be a float, which is valid but always false
    if maybe_prime.as_f64().is_some() {
        response["prime"] = false.into();
        return json::stringify(response).into_bytes();
    }

    return json::stringify(response).into_bytes();
}

fn is_small_prime(val: i64) -> bool {
    if val <= 1 {
        return false;
    }

    let sqrt = f64::sqrt(val as f64) as i64 + 1;

    for n in 2..sqrt {
        if val % n == 0 {
            return false;
        }
    }

    return true;
}

fn is_big_prime(val: BigInt) -> bool {
    // Stolen from https://stackoverflow.com/questions/62150130/algorithm-of-checking-if-the-number-is-prime
    let zero: BigInt = BigInt::from(0);
    let one: BigInt = BigInt::from(1);
    let two: BigInt = BigInt::from(2);
    let three: BigInt = BigInt::from(3);
    let five: BigInt = BigInt::from(5);
    let six: BigInt = BigInt::from(6);

    if val.le(&one) {
        return false;
    }
    if val.le(&three) {
        return true
    }

    if val.modpow(&one, &two).eq(&zero) {
        return false;
    }
    if val.modpow(&one, &three).eq(&zero) {
        return false;
    }

    let sqrt = val.sqrt().add(&one);

    let mut n = five;
    while n.lt(&sqrt) {
        if val.modpow(&one, &n).eq(&zero) {
            return false;
        }

        let nplus2 = (&n).add(&two);
        if val.modpow(&one, &nplus2).eq(&zero) {
            return false;
        }

        n = n.add(&six);
    }

    return true;
}