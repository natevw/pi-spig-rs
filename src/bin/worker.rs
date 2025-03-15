use pi_spig_rs::{comms, spigot};
use std::net::{TcpListener, TcpStream};
use std::{env, process};

fn exit_with_usage(code: i32) -> ! {
    eprintln!("Usage: pi-spig-worker <dst_addr> <port> <offset> <size>");
    process::exit(code);
}

fn main() {
    let dst_addr: &str;
    let src_port: u16;
    let data_offset: usize;
    let data_size: usize;
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        4 => {
            dst_addr = &args[0];
            src_port = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer port");
                    exit_with_usage(1);
                }
            };
            data_offset = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer offset");
                    exit_with_usage(1);
                }
            };
            data_size = match args[3].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer size");
                    exit_with_usage(1);
                }
            };
        }
        _ => {
            //print!("args: {:#?}", args);
            exit_with_usage(-1);
        }
    }

    let mut dst = TcpStream::connect(dst_addr).unwrap();
    let mut spigot = spigot::Spigot::new(data_offset, data_size);
    let listener = TcpListener::bind(("0.0.0.0", src_port)).unwrap();
    let (src, _) = listener.accept().unwrap();
    for n_in in comms::read_all(src) {
        let n_out = spigot.process(n_in);
        comms::send_one(&mut dst, n_out);
    }
}
