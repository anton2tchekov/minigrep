## Minigrep exercise

This is my implementation of the following exercise to learn Rust [web server project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

Author: [Antoine Montiel](mailto:) <br>
Description: simple grep command to find for a string into a document. Returns the lines containing the query.

## Building

Build the project using `make all` (or `make all-dev`).

This calls `cargo build --release` and copies the binary from `target/release` to the root of the repository.

## Usage

Run using `./minigrep bog poem.txt`.

## Project structure

The core of the project lies in `src/lib/`.

## Performance

