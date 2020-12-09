/// # Variables
///
/// Variables in Rust are *immutable* by default, so once a value is assigned you will
/// not be allowed to change this value, unless the var is explicit declared as *mutable*

fn main() {
    // Declaring a variable binding
    let _implicit = "Implict Var Content"; // Without type notation
    let _explicit: &str = "Explicit Var Content"; // Type notaded variable

    // The variables can also de declared as mutable, their type however isn't mutable
    let mut _mutable = 1;
    _mutable = 10;
    _mutable -= 1;

    println!("{}", _implicit); // use {} to print a var
    println!("{1} {0}", _implicit, _explicit); // This can also use index like ordering
    println!("{parameter}", parameter=_mutable); // And a named parameter structure
}
