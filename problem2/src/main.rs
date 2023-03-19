use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream}, 
    thread,
    ops::{Div, AddAssign}
};
use sorted_vec::SortedVec;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PriceChange {
    timestamp: i32,
    price: i32,
}

// impl Ord for PriceChange {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.timestamp.cmp(other.timestamp)
//     }
// }

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
    let mut prices: SortedVec<PriceChange> = SortedVec::new();

    let mut buf = [0; 9];
    loop {
        let res = stream.read_exact(&mut buf);
        if res.is_err() {
            stream.flush().unwrap();
            println!("{id}: read ended");
            break;
        } 
        
        match buf[0] {
            b'I' => {
                let pc = PriceChange{
                    timestamp: i32::from_be_bytes(buf[1..5].try_into().unwrap()),
                    price: i32::from_be_bytes(buf[5..9].try_into().unwrap()),
                };

                //println!("{id}: I {:} {:}", pc.timestamp, pc.price);

                prices.push(pc);
            },
            b'Q' => {
                let from = i32::from_be_bytes(buf[1..5].try_into().unwrap());
                let to = i32::from_be_bytes(buf[5..9].try_into().unwrap());

               // println!("{id}: Q {:} {:}", from, to);

                let avg = calc_avg_price(&prices, from, to);

                //println!("{id}: Q {:} {:} => {:}", from, to, avg);

                stream.write(&avg.to_be_bytes()).unwrap();
            },
            _ => return
        };
    }
}

fn calc_avg_price(prices: &SortedVec<PriceChange>, from: i32, to: i32) -> i32 {
    if from > to {
        return 0
    }
    
    let mut i = 0;
    while i < prices.len() && prices[i].timestamp < from {
        i += 1;
    }

    if i >= prices.len() {
        return 0;
    }

    let mut sum = BigInt::from(0);
    let mut total_prices = 0;

    loop {
        total_prices += 1;
        sum.add_assign(prices[i].price);

        i += 1;
        if i >= prices.len() || prices[i].timestamp > to {
            break;
        }
    }

    let div: BigInt = sum.div(total_prices);
    return div.to_i32().unwrap();
}