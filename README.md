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

# if Cargo are not in PATH, add them
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

Previously: Built out a basic arithmetic lexer and parser.

Currently: The evaluator.

Next: Add support for variables, error-reporting, and let + if statements.

## Developer

Aaron Jin  
[GitHub Profile](https://github.com/aaronkjin)
