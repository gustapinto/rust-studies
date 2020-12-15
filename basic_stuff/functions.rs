/// # Functions
///
/// ## Sumary
///
/// Declaration...............................................................14
/// Return....................................................................25

fn main() { // OBS: main() is the main function :D
    implicit_return();
    function("Two hundred");
    println!("{}", explicit_return());
    println!("I wanna make a supersonic man out of you")
}

/* Declaration
 *
 * Functions are declared using the 'fn' keyword, and they acts like in any
 * other language.
 */
fn function(parameter: &str) {
    println!("{} degrees", parameter);
    println!("That's why they call me Mister Fahrenheit");
}

/* Return
 *
 * Functions in Rust can explicit or implict return a value. If the function
 * explicits return a value its type must be specified.
 */
fn explicit_return() -> String { // Specify function return type
    let string = String::from("I'm traveling at the speed of light");

    return string;
}

fn implicit_return() {
    println!("I'm burning through the sky Yeah!");
}
