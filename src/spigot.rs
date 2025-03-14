use crate::types::RadixDigit;

pub struct Spigot {
    offset: usize,
    array: Box<[RadixDigit]>,
}

impl Spigot {
    pub fn new(offset: usize, size: usize) -> Self {
        Self {
            offset,
            array: vec![2 as RadixDigit; size].into_boxed_slice(),
        }
    }

    pub fn process(&mut self, incoming_carry: RadixDigit) -> RadixDigit {
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

// see https://stackoverflow.com/questions/69051429/what-is-the-function-to-get-the-quotient-and-remainder-divmod-for-rust#comment122040171_69051429
#[inline]
fn divmod(a: RadixDigit, b: RadixDigit) -> (RadixDigit, RadixDigit) {
    (a / b, a % b)
}
