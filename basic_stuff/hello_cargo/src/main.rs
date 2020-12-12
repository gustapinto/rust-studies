/// # A Hello World with cargo
///
/// Cargo is the package manager for Rust, and  is included in the official Rust
/// image.
///
/// To create a Cargo package use ```cargo new package_name```.
/// To build the crated package use ```cargo build package_name```.
/// If you doens't nee d a executable use ```cargo check package_name```, this
///  is much faster than building.
/// And to run the package use ```cargo run package_name```.

fn main() {
    println!("Hello, world!");
}
