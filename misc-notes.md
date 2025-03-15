How I'm testing:

1. fill in `ref.txt` from e.g. <https://www.piday.org/million/> (e.g. "=31415…")
  * or `curl -O https://www.mathsisfun.com/numbers/images/pi-million.txt` is better!
2. `cargo run > test.txt` to start the spigot
  * or now e.g. `time ./target/release/pi-spig-rs 1000000 2 > test.txt`
3. `cmp -i 1 ref.txt test.txt` to check-in
  * or `cmp -i 2 pi-million.txt test.txt` for direct download

Should test it in `--release` but note that this won't panic on overflow:

<https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow>


## Network version

```
cargo build
(
  trap 'kill 0' SIGINT
  target/debug/output 3000 &
  sleep 1; target/debug/worker localhost:3000 3001 0 999 &
  sleep 1; target/debug/zeroes localhost:3001 999 &
)
```

HT: <https://stackoverflow.com/a/52033580>
