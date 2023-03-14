How I'm testing:

1. fill in `ref.txt` from e.g. <https://www.piday.org/million/> (e.g. "=31415…")
2. `cargo run > test.txt` to start the spigot
3. `cmp -i 1 ref.txt test.txt` to check-in

Should test it in `--release` but note that this won't panic on overflow:

<https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow>
