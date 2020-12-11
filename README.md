# PURPOSE & OVERVIEW

This project intends to test the experience of a Live Project of
Manning Publications Co., henceforth referred to as Future Finance
Labs.

The project relies on the Rust language and encourages some discretion
in choosing third party packages/crates. See the `Cargo.toml` and
`Cargo.lock` files to know the specific packages chosen here.

# SETUP

- This is verified correct for Cargo version 1.48.0 (65cbdd2dc 2020-10-14)

# TESTING

- Run tests using a simple command, `cargo test`, to run tests within
  a directory of the project
- To test the app thorough documented use cases
  - Build the app after code changes and before committing
    1. `cargo build`
	2. `cd target/debug/` or `dir .\target\debug\` on Windows
	3. Execute using `./ffl <optional commands>` or on Windows `.\ffl <optional commands>`
  - Refer to CLI documentation to determine correct behavior: `./ffl --help` or on Windows, `.\ffl --help`
  - Test against the 500 symbols
  - Compare, document, and report your findings as appropriate

# CONTRIBUTING

- Unit tests, examples, documentation, and corrections are always appreciated
  - The Rust Team determines the correct standard of code conventions
    etc
- Please send pull requests to the project maintainer, Justin Hale,
  who is user jinhale on
  [Github](https://github.com/jinhale/future-finance-labs/blob/main/src/aggregates/mod.rs)
