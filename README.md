# Tungsten

A compiler built from scratch with no libraries.

Currently building!

## Getting Started

Prerequisites:

```bash
# install Rust
curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs | sh

# check if Cargo is installed
cargo --version

# add if not in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# reload shell config for zsh or bash
source ~/.zshrc
source ~/.bashrc
```

Quick setup to get the compiler running:

```bash
# run
cargo run src/main.rs

# to build, run
cargo build

# for tests, run
cargo test
```

## Progress

Currently supports:

```rust
// var declarations
let a = 10
let b = true

// arithmetic expressions
let res = (a + 5) * 2

// func declaraations
func add(x, y) {
    return x + y
}

// func calls
let sum = add(1, 2)

// conditional statements
if a > 5 {
    a = 25
} else {
    a = 20
}

// while loops
while a < 10 {
    a = a + 1
}

// block scoping
{
    let local_var = 15
}
```

## Developer

Aaron Jin  
[GitHub Profile](https://github.com/aaronkjin)
