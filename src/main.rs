use pi_spig_rs::{display, spigot};
use std::{env, io::stdout, process, sync::mpsc, thread};

fn exit_with_usage(code: i32) -> ! {
    eprintln!("Usage: pi-spig-rs {{<n_digits>}} {{<n_threads>}}");
    process::exit(code);
}

fn main() {
    let n_digits: usize;
    let n_threads: usize;
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        2 => {
            n_digits = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer n_digits");
                    exit_with_usage(1);
                }
            };
            n_threads = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer n_threads");
                    exit_with_usage(1);
                }
            };
        }
        _ => {
            //print!("args: {:#?}", args);
            exit_with_usage(-1);
        }
    }

    let mut output_dest = stdout().lock();
    let (tx_main, rx_main) = mpsc::channel();

    let arr_len: usize = (10 * n_digits / 3) + 1;
    let arr_len_per_thread: usize = arr_len / n_threads;

    let mut tx_next = tx_main;
    for i in 0..n_threads {
        let (tx_self, rx_self) = mpsc::channel();
        thread::spawn(move || {
            let mut spigot = spigot::Spigot::new(i * arr_len_per_thread, arr_len_per_thread);
            for q_prev in rx_self {
                let q_self = spigot.process(q_prev);
                tx_next.send(q_self).unwrap();
            }
        });
        tx_next = tx_self;
    }
    thread::spawn(move || {
        for _ in 0..n_digits {
            tx_next.send(0).unwrap();
        }
    });

    let mut display = display::OutputDisplay::new(&mut output_dest);
    for q in rx_main {
        display.push_for_release(q);
    }
    display.push_for_release(0);
    println!("");
}
