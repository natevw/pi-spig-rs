use std::{io::{stdout, Write}, thread, sync::mpsc};

type OutputDigit = u8; // increase if converting to higher (e.g. 1000) base
type RadixDigit = u32; // TODO: analyze when/if this could overflow

fn main() {
    const N_DIGITS: usize = 1000000;
    let mut output_dest = stdout().lock();

    const ARR_LEN: usize = (10 * N_DIGITS / 3) + 1;
    
    
    let (tx_main, rx_main) = mpsc::channel();
    const N_WORK_THREADS: usize = 2;
    const ARR_LEN_PER_THREAD: usize = ARR_LEN / N_WORK_THREADS;
    
    let mut tx_next = tx_main;
    for i in 0..N_WORK_THREADS {
        let (tx_self, rx_self) = mpsc::channel();
        thread::spawn(move || {
            let mut spigot = Spigot::new(i * ARR_LEN_PER_THREAD, ARR_LEN_PER_THREAD);
            for q_prev in rx_self {
                let q_self = spigot.process(q_prev);
                tx_next.send(q_self).unwrap();
            }
        });
        tx_next = tx_self;
    }
    thread::spawn(move || {
        for _ in 0..N_DIGITS {
            tx_next.send(0).unwrap();
        }
    });
    
    
    let mut first_held: OutputDigit = 0;
    let mut num_held_nines: usize = 0;
    let mut push_for_release = |outgoing: RadixDigit| {
        if outgoing == 9 {
            num_held_nines += 1;
            return;
        } else if outgoing < 9 {
            // HT: https://stackoverflow.com/a/35280799/179583
            write!(output_dest, "{}{:9<2$}", first_held, "", num_held_nines).unwrap();
            first_held = outgoing as OutputDigit;
        } else {
            assert!(outgoing == 10);
            assert!(first_held < 9);
            write!(output_dest, "{}{:0<2$}", first_held + 1, "", num_held_nines).unwrap();
            first_held = 0;
        }
        num_held_nines = 0;
        output_dest.flush().unwrap();
    };
    
    for q in rx_main {
        push_for_release(q);
    }
    push_for_release(0);
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
