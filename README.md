# PURPOSE & OVERVIEW

This project intends to test the experience of a Live Project of
Manning Publications Co., henceforth referred to as Future Finance
Labs.

The project relies on the Rust language and encourages some discretion
in choosing third party packages/crates. See the `Cargo.toml` and
`Cargo.lock` files to know the specific packages chosen here.

# SETUP


- This is verified correct for Cargo version 1.48.0 (65cbdd2dc 2020-10-14)
- Run `rustup target add x86_64-pc-windows-msvc`, wich corresponds to
  the triple of `utils/Cargo.toml`
- Run `cargo build`
- Run `./target/debug/ffl-cli` to run the command
  - Sample command and result:

~~~~
$ ./target/debug/ffl-cli -t GOOG,IBM
period start,symbol,price,change %,min,max,30d avg
2020-11-02T14:30:00+00:00,GOOG,$134.03,0.08%,$1616.03,$1847.20,$1766.24,
period start,symbol,price,change %,min,max,30d avg
2020-11-02T14:30:00+00:00,IBM,$12.23,0.11%,$111.16,$127.69,$119.79,
~~~~

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

# Benchmarks

~~~~
	 Running target/release/deps/bench-fe57e3aedf005419
Gnuplot not found, using plotters backend
async min               time:   [3.0547 ns 3.0713 ns 3.0899 ns]
						change: [-2.1907% -0.8783% +0.3533%] (p = 0.18 > 0.05)
						No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

async max               time:   [3.2031 ns 3.2347 ns 3.2721 ns]
						change: [+6.0433% +7.9078% +9.6620%] (p = 0.00 < 0.05)
						Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

async change            time:   [3.0310 ns 3.0438 ns 3.0579 ns]
						change: [-0.1136% +0.8258% +1.7266%] (p = 0.09 > 0.05)
						No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

async n_window_sma      time:   [3.2777 ns 3.2886 ns 3.3002 ns]
						change: [+0.5424% +1.5068% +2.4338%] (p = 0.00 < 0.05)
						Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
~~~~
