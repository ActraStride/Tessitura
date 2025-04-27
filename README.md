# Tessitura

A simple Rust utility for reading and displaying the contents of files in a directory.

## Description

Tessitura is a command-line tool that reads all files in a specified directory and prints their contents to the console. It's a straightforward utility for quickly reviewing the contents of multiple files at once.

## Features

- Reads all files in a specified directory
- Displays file paths and their contents
- Handles IO errors gracefully

## Installation

Make sure you have Rust installed on your system. You can install it from [rustup.rs](https://rustup.rs/).

Clone this repository and build the project:

```bash
git clone https://github.com/yourusername/tessitura.git
cd tessitura
cargo build --release
```

## Usage

Run the application with:

```bash
cargo run
```

By default, it will read files from the current directory. To specify a different directory, modify the `directorio` variable in `main.rs`.

## Development

This project is built with Rust and uses the standard library's file system and IO modules.

## License

...