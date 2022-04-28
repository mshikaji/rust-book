# rust-book
code &amp; doc based on "The Rust Programming Language" (https://doc.rust-lang.org/stable/book/)

##`cargo` intro
* `cargo new <project name>` creates a new rust project and associated cargo files. `Cargo.toml` file contains project and dependency info
* `cargo build` builds the project binaries and places them in `target\debug`
* `cargo run` runs executables in `target\debug` (how do you run executables in `target\release`?)
* `cargo check` builds the project *without* producing binaries (speeds development FTW!)
* `cargo build --release` builds *optimized* binaries and places them in `target\release`

##Guessing Game
* Rust imports some items (from the standard library?) into every program by default, this is called *the prelude*
* `use` imports non-prelude items from libraries
* a no-arg `main()` means no parameters
* `let mut guess = String::new();` creates *mutable* variable `guess` and binds it to a new string instance
* `String::new()` means call the `new()` *associated* function on the `String` type
* the function call `io::stdin()` can also be `std::io::stdin()` if there is no `use std::io` import
* `io::stdin()` returns a stdin handle
* `.read_line(&mut guess)` reads a line from stdin and *appends* it to the passed string; that's why it must be mutable reference (`&mut`)
* the `read_line` function returns an `io::Result` enumeration whose variants are `Ok` and `Err`; the `expect` method returns the contained value if `Ok` or halt the program and display the error message if `Err`
* handling of the `io::Result` enumeration is enforced by the compiler
* we need the `rand` crate, so added `rand = "0.8.3"` dependecy in `Cargo.toml`; cargo downloads the latest *patch* version
* `Cargo.lock` is created to freeze the project at certain versions; an update to `Cargo.toml` is required to have a newer major or minor version downloaded
* `cargo doc --open` generates documentation for a project and opens it in a browser
