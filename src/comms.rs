use crate::types;
use std::io::{Read, Write};
use std::iter;

pub fn read_all(mut src: impl Read) -> impl iter::Iterator<Item = types::RadixDigit> {
    iter::from_fn(move || {
        let mut buffer = [0u8; std::mem::size_of::<types::RadixDigit>()];
        match src.read_exact(&mut buffer) {
            Ok(_) => Some(types::RadixDigit::from_be_bytes(buffer)),
            Err(_) => None,
        }
    })
}

pub fn send_one(dst: &mut impl Write, n: types::RadixDigit) {
    let d = n.to_be_bytes();
    dst.write(&d).unwrap();
}
