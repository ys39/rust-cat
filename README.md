# Implementation of the cat command

## Usage/Examples

```rust
Options:
  -A, --show-all          equivalent to -vET
  -b, --number-nonblank   number nonempty output lines, overrides -n
  -e                      equivalent to -vE
  -E, --show-ends         display $ at end of each line
  -n, --number            number all output lines
  -s, --squeeze-blank     suppress repeated empty output lines
  -t                      equivalent to -vT
  -T, --show-tabs         display TAB characters as ^I
  -u                      (ignored)
  -v, --show-nonprinting  use ^ and M- notation, except for LFD and TAB
```

```rust
cargo build --release

./target/release/mycat
./target/release/mycat -n
./target/release/mycat -n -E
```
