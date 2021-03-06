/// # Primitive Types
///
/// ## Sumary
///
/// Boolean...................................................................11
/// Text......................................................................18
/// Numbers...................................................................32
/// Data Structures...........................................................51

fn main() {
    /* Boolean
     *
     * Rust use the 'true' and 'false' Booleans to express truly values.
    */
    let _true: bool = true;
    let _false: bool = false;

    /* Text
     *
     * Rust has three major text types:
     *
     * String (String)     -> A String is derivated from the String object and
     *                         it's a mutable growable UTF-8 encoded collection.
     * String Slice (&str) -> A String Slice is static string literal.
     * Character (char)    -> A Character is a single 4 bytes literal, they are
     *                         declared with single quotes 'char'.
    */
    let _string = String::new();
    let _string_slice : &str = "Galileo! Galileo!";
    let _char : char = 'A';

    /* Numbers
     *
     * Rust provides the most commom types to represent numbers that follow a
     * simple naming rule sufix'n' where 'n' represents the power of 2 that
     * corresponds to the max number size. Example: sufix8 has a maximum size of
     * 2^8 bytes. 'n' is defined as a doubling multiple of 8 between 8 and 64,
     * so the possible values to 'n' are: 8, 16, 32, and 64.
     *
     * Signed Integers (i'n')   -> A Signed Integer can receive either positive
     *                              or negative values.
     * Unsigned Integers (u'n') -> A Unsigned Integer can only receive positive
     *                              values.
     * Floats (f'n')            -> Floats can be either positive or negative and
     *                              can only use the 32 and 64 'n' values.
    */
    let _signed_int: i8 = -1;
    let _unsigned_int: u8 = 1;
    let _float: f32 = -1.11;

    /* Data Structures
     *
     * Rust has two data structures:
     *
     * Array -> The Array is a collection of imutable homogeneous elements in a
     *           memory location that can be acessed individually.
     * Tuple -> A Tuple is a sequence of static items or elements, once created
     *           the tuple elements and size can't be changed.
    */
    let _array = ["Galileo!", "Galileo!"];
    let _tuple = ("Galileo", "Figaro!");
}
