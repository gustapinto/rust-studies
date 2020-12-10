/// # Variables
///
/// Variables in Rust are immutable by default, so once a value is assigned you
/// will not be allowed to change this value unless the var is explicit declared
/// as mutable

fn main() {
    // Declaring a variable binding
    let _implicit = "Implict Var Content"; // Without type notation
    let _explicit: &str = "Explicit Var Content"; // Type notaded variable

    let mut _mutable: u8 = 1; // The type of a mutable variable isn't mutable.
    _mutable = 10;
    _mutable -= 1;

    println!("{}", _implicit); // use {} to print a var
    println!("{1} {0}", _implicit, _explicit); // Index like {} ordering
    println!("{parameter}", parameter=_mutable); // named {} parameter structure
}
