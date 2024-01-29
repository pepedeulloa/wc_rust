# wc_rust

**wc_rust** is a Rust implementation of the `wc` command, a standard Unix utility for counting words, lines, chars and bytes of a file.

## Dependencies

This program has been developed using Rust version 1.74.0. Command-line argument management has been simplified thanks to the use of the crate clap in its version 4.4.11. clap offers a declarative interface for defining and parsing program arguments, making it easy to create a friendly and efficient user experience.

## Installation

1. Clone the repository: `git clone https://github.com/tuusuario/wc_rust.git`
2. Navigate to the project directory: `cd wc_rust`
3. Compile the project: `cargo build --release`

## Options

- `-c, --bytes`: Print bytes count of the file
- `-l, --lines`: Print lines count of the file
- `-m, --chars`: Print character count of the file
- `-w, --words`: Print words count of the file
- `-h, --help`: Show the program's help.
- `-V, --version`: Show the program's version.

## Usage

To use `wc_rust`, you can compile the program and run it from the command line. It supports various options to customize the output:

```bash
# Example usage with options
cd target/release
./wc_rust -cl file1.txt file2.txt
```
