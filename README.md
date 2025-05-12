# Tungsten

A compiler, except build-it-yourself version. And by yourself, I mean me.

Currently building 👷‍♂️

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

Previously:

- Lexer
- Parser
- Evaluator
- Error-reporter
- Support for `let` statements, binary operators, unary expressions

Currently: If/while statements.

Next: Scoping.

## Developer

Aaron Jin  
[GitHub Profile](https://github.com/aaronkjin)
