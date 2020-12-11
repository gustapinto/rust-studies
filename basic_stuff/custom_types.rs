/// # Primitive Types
///
/// ## Sumary
///
/// Structures................................................................10
/// Enumerations..............................................................50
/// Constants.................................................................73

fn main() {
    /* Structures
     *
     * Structures are Rust equialents to hashmaps, acting as named tuples, they
     * can be divided into three main types, the 'Tuple Structs', C Structs and
     * the 'Unit Structs'.
     * They are also equivalents for classes, but instead of having a lot of
     * self properties and methods they acts as a dump data structure with their
     * methods being implemented by the 'impl' keyword.
     *
     * Tuple Structs -> The main struct type in Rust, they serve basically as
     *                   named tuples.
     * C Structs     -> 'C' like structs, with a type-variable structure.
     * Unit Structs  -> Field-less structs, used for generics.
    */
    struct TupleStruct { // Initialize a normal struct
        field: String,
    }

    struct UnitStruct; // A Unit Struct, without any fields

    let tuple_struct = TupleStruct {
        field: "Some Field".to_string()
    };
    let _unit_struc = UnitStruct;

    println!("{}", tuple_struct.field);

    impl TupleStruct { // Implements methods to TupleStruct{}
        fn new(field: &str) -> Self { // Constructor
            Self { field: String::from(field) }
        }

        fn show_field(&self) {
            println!("{}", self.field);
        }
    }

    let new_tuple_struct = TupleStruct::new("Another Field");
    new_tuple_struct.show_field();

    /* Enumerations
     *
     * Rust Enums are used to define a type by enumerating its possible variants
     * creating custom data types that can be use anywhere on the script.
     * Enums can make use of the 'use' keyword to be called without scoping.
    */
    enum CustomEnum {
        Type1,
        Type2,
    }

    let _type_1_var = CustomEnum::Type1;
    let _type_2_var = CustomEnum::Type2;

    fn inspect(event: CustomEnum) { // Using enums to beuild a match
        match event { // The match uses Enum elements to return values
            CustomEnum::Type1 => println!("Type 1"),
            CustomEnum::Type2 => println!("Type 2"),
        }
    }

    inspect(CustomEnum::Type1);

    /* Constants
     *
     * Rust has two types of constants, they can be declared in any scope and
     * bothe require explicite annotation.
     *
     * Const (const)   -> A variable with unchangeable value.
     * Static (static) -> A possibly mutable variable with a static lifetime,
     *                     that is inferred and doesn't need to be specified.
     *                     OBS: Acessing a mutable static is 'unsafe'.
    */
    const _CONSTANT: &str = "A unchageable string";
    static _STATIC: &str = "A maybe mutable string";
}
