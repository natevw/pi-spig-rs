use pi_spig_rs::{display, types};
use std::{env, io::stdout, io::Read, net::TcpListener, process};

fn exit_with_usage(code: i32) -> ! {
    eprintln!("Usage: pi-spig-display <port>");
    process::exit(code);
}

fn display_from(src: &mut impl Read) {
    let values = std::iter::from_fn(move || {
        let mut buffer = [0u8; std::mem::size_of::<types::RadixDigit>()];
        match src.read_exact(&mut buffer) {
            Ok(_) => Some(types::RadixDigit::from_be_bytes(buffer)),
            Err(_) => None,
        }
    });

    let mut output_dest = stdout().lock();
    let mut display = display::OutputDisplay::new(&mut output_dest);
    for q in values {
        display.push_for_release(q);
    }
    display.push_for_release(0);
    println!("");
}

fn main() {
    let port: u16;
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        1 => {
            port = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer n_digits");
                    exit_with_usage(1);
                }
            };
        }
        _ => {
            //print!("args: {:#?}", args);
            exit_with_usage(-1);
        }
    }

    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    match listener.accept() {
        Ok((mut socket, addr)) => {
            println!("connected: {addr:?}");
            display_from(&mut socket);
        }
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}
