# [minigrep](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

This is a simple command-line tool written in Rust that mimics the basic functionality of the Unix `grep` command. It searches for a specified query string within a given file and prints out the lines that contain the query.

## Usage

```bash
cargo run -- <query> <file_path>
```

You can try it out with the following command:

```bash
cargo run -- to poem.txt
```
