- `cargo new <project-name>` for binary project and with `--lib` for library project
- `cargo init` to initialize a new cargo project in an existing directory
  - also works with `--lib` flag
- `cargo build` / `cargo build --release`
- `cargo run` / `cargo run --release`
- `cargo check`
- `cargo doc --open`
- `cargo test`
  - see `src/lib.rs` for examples of writing unit tests and `tests/*` for integration tests
  - default is `parallel` testing, use `-- --test-threads=1` for sequential testing
  - `-- --show-output` to see `println!` output in tests
  - `-- --ignore` to ignore specific test unless explicitly called
  - `-- --include-ignored` to include ignored tests
  - `cargo test <test-function-name>` to run specific test for unit tests
  - `cargo test --test <test-file-name>` to run specific test file for integration tests


- rust [std](https://doc.rust-lang.org/std/prelude/index.html) library
