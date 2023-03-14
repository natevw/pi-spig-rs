use std::io::{stdout, Write};

type OutputDigit = u8;  // increase if converting to higher (e.g. 1000) base
type RadixDigit = u32;  // TODO: analyze when/if this could overflow

fn main() {
    const N_DIGITS: usize = 5000000;
    let mut output_dest = stdout().lock();
    
    let arr_len = (10 * N_DIGITS / 3) + 1;
    let mut arr = vec![2 as RadixDigit; arr_len];
    
    let mut first_held: OutputDigit = 0;
    let mut num_held_nines: usize = 0;
    let mut push_for_release = |outgoing: RadixDigit| {
        if outgoing == 9 {
            num_held_nines += 1;
            return
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
    
    for _ in 0..N_DIGITS {
        for a_idx in 0..arr_len {
            arr[a_idx] *= 10;
        }
        for a_idx in (1..arr_len).rev() {
            // this stuff following my Python version for now
            let i: RadixDigit = (a_idx as RadixDigit) + 1;
            let modulo: RadixDigit = 2*i - 1;
            let q = arr[a_idx] / modulo;
            let r = arr[a_idx] % modulo;
            arr[a_idx] = r;
            arr[a_idx - 1] += q * (i - 1);
        }
        let q = arr[0] / 10;
        let r = arr[0] % 10;
        arr[0] = r;
        push_for_release(q)
    }
    push_for_release(0);
    println!("");
}
