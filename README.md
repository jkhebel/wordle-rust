# Wordle x Rust

## About

This is a simple wordle clone I threw together to practice and learn some basic Rust. My goal is to eventually flesh it out with additional modes, customization, a GUI, and interfacing for testing ML solutions.

## Dependencies

This project was inspired by [Wordle](https://www.rust-lang.org/) and built using [Rust](https://www.rust-lang.org/).

## Installation

To install the game, download the binary from the latest [release](https://github.com/jkhebel/wordle-rust/releases). If you'd rather build from source, be sure to follow the official guides for installing `rust` and `cargo` using `rustup` before cloning the repository. You can then build the source file from scratch using:
```
>cargo build --release
```

Or build and run the game using:
```
>cargo run
```

## Getting Started

Launching the game is easy! Simply navigate to the directory containing the binary and run:
```
>./wordle
```

If emoji aren't rendering correctly in your terminal, you can pass in the `--no-emoji` flag to replace then with standard characters.

```
>./wordle --no-emoji
```

Additional command line arguments can be viewed using the `--help` flag.

```
>/.wordle --help
```


Enjoy a never ending loop of wordle!

## Roadmap

- Ability to choose word length and number of tries
- Additional language support
- Dictionary definitions for language learning
- A simple GUI in rust
- Bindings for training ML solvers
