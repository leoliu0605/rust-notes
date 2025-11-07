# Rust Notes

This is the repository of quick study notes for the [Rust Book](https://doc.rust-lang.org/stable/book/).

## Need More Time to Digest

- What is `'a` meaning in [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- Deeply understand [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) when needed

## Cargo Commands

- `cargo new <project-name>` for binary project and with `--lib` for library project
- `cargo init` to initialize a new cargo project in an existing directory
  - also works with `--lib` flag
- `cargo build` / `cargo build --release`
- `cargo run` / `cargo run --release`
- `cargo check`
- `cargo doc --open`
- `cargo test`
  - see [`src/lib.rs`](./src/lib.rs) for examples of writing unit tests and [`tests`](./tests) for integration tests
  - default is `parallel` testing, use `-- --test-threads=1` for sequential testing
  - `-- --show-output` to see `println!` output in tests
  - `-- --ignore` to ignore specific test unless explicitly called
  - `-- --include-ignored` to include ignored tests
  - `cargo test <test-function-name>` to run specific test for unit tests
  - `cargo test --test <test-file-name>` to run specific test file for integration tests
- `cargo publish` to publish the crate to [crates.io](https://crates.io/)
  - make sure to update version in `Cargo.toml` before publishing
  - `cargo yank --vers <version>` to yank a specific version to prevent new projects from using it
- `cargo install <crate-name>` to install a binary crate from [crates.io](https://crates.io/)
  - use `--force` to reinstall or update an already installed crate
  - more info: [cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html)
- `cargo add <dependency>` to add a dependency to `Cargo.toml`
  - requires `cargo-edit` tool, install with `cargo install cargo-edit`
- `cargo update` to update dependencies in `Cargo.lock` to the latest allowed versions based on `Cargo.toml`
- `cargo tree` to visualize the dependency graph of the project
  - requires `cargo-tree` tool, install with `cargo install cargo-tree`
- `cargo fmt` to format the code using `rustfmt`
- `cargo fix` to automatically fix common issues
- `cargo clippy` to run the Clippy linter for catching common mistakes and improving code quality

## Useful Links

- rust [std](https://doc.rust-lang.org/std/prelude/index.html) library
- Read more about [workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for managing multiple related packages

## More ...

- See [minigrep](./minigrep) for a simple command-line tool example that show how to use Rust for file I/O and string manipulation.
- See [hello-async](./hello-async) for a simple example of using async-await in Rust.
