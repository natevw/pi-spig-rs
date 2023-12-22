use std::{io::{stdout, Write}, thread, sync::mpsc, env, process};

type OutputDigit = u8; // increase if converting to higher (e.g. 1000) base
type RadixDigit = u32; // TODO: analyze when/if this could overflow

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
                },
            };
            n_threads = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Expected integer n_threads");
                    exit_with_usage(1);
                },
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
            let mut spigot = Spigot::new(i * arr_len_per_thread, arr_len_per_thread);
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
    
    let mut display = OutputDisplay::new(&mut output_dest);
    for q in rx_main {
        display.push_for_release(q);
    }
    display.push_for_release(0);
    println!("");
}

// see https://stackoverflow.com/questions/69051429/what-is-the-function-to-get-the-quotient-and-remainder-divmod-for-rust#comment122040171_69051429
#[inline]
fn divmod(a: RadixDigit, b: RadixDigit) -> (RadixDigit, RadixDigit) {
    (a / b, a % b)
}

struct Spigot {
    offset: usize,
    array: Box<[RadixDigit]>,
}

impl Spigot {
    fn new(offset: usize, size: usize) -> Self {
        Self {
            offset,
            array: vec![2 as RadixDigit; size].into_boxed_slice(),
        }
    }

    fn process(&mut self, incoming_carry: RadixDigit) -> RadixDigit {
        let arr = &mut self.array;
        let mut q = incoming_carry;
        for (arr_idx, digit) in arr.iter_mut().enumerate().rev() {
            let overall_idx = self.offset + arr_idx;
            let (modulus, qumerator) = if overall_idx == 0 {
                (10, 1)
            } else {
                let i = (overall_idx as RadixDigit) + 1;
                (2 * i - 1, i - 1)
            };
            
            let r: RadixDigit;
            let adj_digit = *digit * 10 + q;
            (q, r) = divmod(adj_digit, modulus);
            q *= qumerator;
            *digit = r;
        }
        q
    }
}

struct OutputDisplay<'w> {
    output: &'w mut dyn Write,
    first_held: OutputDigit,
    num_held_nines: usize,
}

impl<'w> OutputDisplay<'w> {
    fn new(destination: &'w mut dyn Write) -> Self {
        Self { output: destination, first_held: 0, num_held_nines: 0 }
    }
    
    fn push_for_release(&mut self, outgoing: RadixDigit) {
        let nh9 = &mut self.num_held_nines;
        let fh = &mut self.first_held;
        if outgoing == 9 {
            *nh9 += 1;
            return;
        } else if outgoing < 9 {
            // HT: https://stackoverflow.com/a/35280799/179583
            write!(self.output, "{}{:9<2$}", *fh, "", *nh9).unwrap();
            *fh = outgoing as OutputDigit;
        } else {
            assert!(outgoing == 10);
            assert!(*fh < 9);
            write!(self.output, "{}{:0<2$}", *fh + 1, "", *nh9).unwrap();
            *fh = 0;
        }
        *nh9 = 0;
        self.output.flush().unwrap();
    }
}