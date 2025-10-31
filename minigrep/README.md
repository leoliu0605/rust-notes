# [minigrep](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

This is a simple command-line tool written in Rust that mimics the basic functionality of the Unix `grep` command. It searches for a specified query string within a given file and prints out the lines that contain the query.

## Usage

```bash
cargo run -- <query> <file_path>
```

You can try it out with the following command:

```bash
cargo run -- body poem.txt
#    Compiling minigrep v0.1.0 (/Users/leoli/Desktop/SideProject/rust/minigrep)
#     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
#      Running `target/debug/minigrep body poem.txt`
# Searching for 'body'
# In file 'poem.txt'
# I'm nobody! Who are you?
# Are you nobody, too?
# How dreary to be somebody!
cargo run -- monomorphization poem.txt
#     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
#      Running `target/debug/minigrep monomorphization poem.txt`
# Searching for 'monomorphization'
# In file 'poem.txt'
```

```bash
cargo run -- to poem.txt
#     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
#      Running `target/debug/minigrep to poem.txt`
# Searching for 'to'
# In file 'poem.txt'
# Are you nobody, too?
# How dreary to be somebody!
IGNORE_CASE=1 cargo run -- to poem.txt
#    Compiling minigrep v0.1.0 (/Users/leoli/Desktop/SideProject/rust/minigrep)
#     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
#      Running `target/debug/minigrep to poem.txt`
# Searching for 'to'
# In file 'poem.txt'
# Are you nobody, too?
# How dreary to be somebody!
# To tell your name the livelong day
# To an admiring bog!
```
