use std::{
    io::{prelude::*, BufReader},
    env,
    net::{TcpStream}, 
    fs::File,
};


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut client = TcpStream::connect("127.0.0.1:8080")?;

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for hex in line?.split(" ") {
            let byte = u8::from_str_radix(hex, 16).unwrap();

            client.write(&[byte])?;
        }
    }

    let mut buf = [0; 8];
    loop {
        let n = client.read(&mut buf)?;

        for b in &buf[..n] {
            print!("{:02X} ", b);
        }
        println!();

        if n != 8 {
            break;
        }
    }

    Ok(())
}
