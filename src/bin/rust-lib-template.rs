//! Thin binary wrapper around the library.
//!
//! This binary is optional — the primary deliverable
//! is the library crate. Install with `cargo install`.

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).map(|s| s.as_str()).unwrap_or("");

    match rust_lib_template::process(input) {
        Ok(output) => println!("{output}"),
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}
