# which-cli

A Rust CLI equivalent of Unix `which(1)`, built with the [`which`](https://crates.io/crates/which) crate.

## Build

```bash
cargo build --release
```

## Usage

```bash
./target/release/which <command>...
```

## Examples

```bash
# Find single command
which cat
# Output: /usr/bin/cat

# Find multiple commands
which zed cat
# Output:
# /mnt/c/Users/qiuzh/AppData/Local/Programs/Zed/bin/zed
# /usr/bin/cat
```
