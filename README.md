# Tungsten

A compiler, except build-it-yourself version. And by yourself, I mean me.

Currently building 👷‍♂️

## Getting Started

Prerequisites:

```bash
# ensure Rust is installed; if not, install
curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs | sh

# ensure Cargo is installed
cargo --version

# if Rust and Cargo are not in PATH, add them
export PATH="$HOME/.cargo/bin:$PATH"

# reload shell configuration
source ~/.zshrc   # For zsh
source ~/.bashrc  # For bash
```

Quick setup to get the compiler running:

```bash
# go into crust directory
cd crust

# run Cargo
cargo run src/main.rs
```

## Progress

Previously: Built out a basic lexer.

Currently: The parser.

Next: Error-reporting.

## Developer

Aaron Jin  
[GitHub Profile](https://github.com/aaronkjin)
