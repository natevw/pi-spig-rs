use pi_spig_rs::comms;
use std::{env, net::TcpStream, process};

fn exit_with_usage(code: i32) -> ! {
    eprintln!("Usage: pi-spig-zeroes <addr> <count>");
    process::exit(code);
}

fn main() {
    let addr: &str;
    let count: usize;
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        2 => {
            addr = &args[0];
            count = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer count");
                    exit_with_usage(1);
                }
            };
        }
        _ => {
            //print!("args: {:#?}", args);
            exit_with_usage(-1);
        }
    }

    let mut socket = TcpStream::connect(addr).unwrap();
    for _ in 0..count {
        comms::send_one(&mut socket, 0);
    }
}
