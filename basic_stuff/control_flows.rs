/// # Control Flows
///
/// ## Sumary
///
/// If, Else..................................................................13
/// Loop......................................................................37
/// While.....................................................................74
/// For.......................................................................84

fn main() {
    let number: f32 = get_user_input_as_float();

    /* If, Else
     *
     * If and Else acts as truly conditional operators, they can also be used
     * with 'let' to build a ternary operator on variable assignment.
    */
    if number < 0.0 { // Traditional If/Else
        println!("{} is negative", number);
    } else if number > 0.0 {
        println!("{} is positive", number);
    } else {
        println!("{} is zero", number);
    }

    let new_number = // Ternary
        if number < 0.0 {
            10
        } else {
            1
        };

    println!("New number: {}", new_number);

    println!("");

    /* Loop
     *
     * The loop statement is use to represent infinite loops, they can be nested
     * using annotations on loop declaration.
    */
    let mut loop_count: u32 = 0;

    loop { // Basic Loop
        loop_count += 1;

        println!("Another one bites the dust");

        if loop_count == 2 {
            println!("And another one gone, and another one gone");

            continue;
        }

        if loop_count == 3 {
            println!("Hey, I'm gonna get you too")
        }

        if loop_count == 4 {
            break println!(""); // To return something from loop put on a break
        }
    }

    'nested: loop { // Nested Loop
        'inner: loop {
            break 'inner;
        }

        break 'nested;
    }

    println!("");

    /* While
     *
     * While is responsable for running loops while a certain condition is truly
    */
    let mut while_count: u32 = 0;

    while while_count <= 5 {
        while_count += 1;
    }

    /* For
     *
     * The 'for' keyword is used to iterate over a Iterator, like a array or over
     * a range notation (start..exclusive_end) or (start..=inclusive_end). While
     * itering over Iterators the 'for' has multiple means to interact with the
     * elements of the collection:
     *
     * Iterator.iter()      -> This borrows each element of the collection over
     *                          each iterarion, leaving the original collection
     *                          untouched and available for reuse after the loop.
     * Iterator.into_iter() -> This consumes the collection on each iteraration
     *                          so the exact data is provided, after the
     *                          iteration the collection is no longer available
     *                          for reuse.
     * Iterator.iter_mut()  -> This mutably borrows each collection elements,
     *                          allowing it to be modified.
    */
    for _ in 1..5 { // Iterate over the 1-5 range, excluding the end
        println!(".");
    }

    for _ in 5..=10 { // Iterate over the range including the end
        println!(".");
    }

    println!("");

    let _array = vec![
        "All we hear is Radio Ga Ga",
        "Radio Goo Goo",
        "Radio Blah Blah"
    ];
    let mut _mut_array =  vec![
        "All we hear is Radio Ga Ga",
        "Radio Blah Blah",
        "Radio, what's new?",
        "missing entry"
    ];

    for lyric in _array.iter() { // Iterating using .iter()
        // Rust provides a pattern matching via the 'match' keyword:
        match lyric { // Note that the match {} changes with each iteration type
            &"Radio Blah Blah" => println!("Radio Ga Ga"),
            _ => println!("{}", lyric),
        }
    }

    for lyric in _array.into_iter() { // Iterating using .into_iter()
        match lyric {
            "Radio Blah Blah" => println!("Radio Ga Ga"),
            _ => println!("{}", lyric),
        }
    }

    println!("");

    for lyric in _mut_array.iter_mut() {
        *lyric = match lyric {
            &mut "missing entry" => "Radio, someone still loves you",
            _ => continue,
        }
    }

    println!("{:?}", _mut_array);

}

fn get_user_input_as_float() -> f32 { // Setting to return a f32 float
    /* Getting the user Input
     *
     * Rust make use of std::io libray to get input from terminal.
     *
     * These librarys will be covered in another example.rs.
    */
    use std::io; // Rust std I/O library

    let mut input = String::new();

    println!("Type a number: ");
    io::stdin().read_line(&mut input).unwrap();

    return input.trim().parse().unwrap();
}
