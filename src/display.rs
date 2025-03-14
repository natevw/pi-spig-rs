use crate::types::{OutputDigit, RadixDigit};
use std::io::Write;

pub struct OutputDisplay<'w> {
    output: &'w mut dyn Write,
    first_held: OutputDigit,
    num_held_nines: usize,
}

impl<'w> OutputDisplay<'w> {
    pub fn new(destination: &'w mut dyn Write) -> Self {
        Self {
            output: destination,
            first_held: 0,
            num_held_nines: 0,
        }
    }

    pub fn push_for_release(&mut self, outgoing: RadixDigit) {
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
