/* CONTROL FLOW */
fn main() {
    // If-Else
    let number = 8;
    if number < 5 {
        println!("number less than 5");
    } else {
        println!("number greater than 5");
    }

    // If-Else if
    if number % 4 == 0 {
        println!("number divisible by 4!");
    } else if number % 3 == 0 {
        println!("number is divisible by 3!");
    } else if number % 2 == 0 {
        println!("number is divisible by 2!");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 'if' can be used in 'let' statements
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // 'loop' keyword functions
    loops();

    // 'while' keyword functions
    whiles();

    // 'for' keyword functions
    for_loop();
}

fn loops() {
    /* Rust has three kinds of loops:
     * - loop
     * - while
     * - for
     */
    let mut i = 0;
    // 'loop' loops forever or until broken manually
    loop {
        println!("loop!");
        if i == 5 {
            break;
        }
        i += 1;
    }

    /* Values can be returned from loops!
     * One reason to do this is to retry an operation
     * you know might fail, such as checking if a
     * thread has finished executing. */
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // This will return 20 from the loop
        }
    }; // Add semicolon because it is a statement now, not an expression
    println!("The result is {result}");

    /* If you have loops within loops, 'break' and
     * 'continue' apply to the innermost loop at that
     * point. Optionally, a "loop label" can be specified
     * for a loop that can be paired with 'break' or
     * 'continue' to apply to the labeled loop no matter
     * where the keywords are used.
     * Loop labels must begin with a single quote. */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Break the 'counting_up loop from the inner loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whiles() {
    // The 'while' keyword enables conditional loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    // Loop through every element in the array and print it
    for element in a {
        println!("the value is: {element}");
    }

    /* The countdown from the whiles() function can be implemented
     * using a Range - which produces all numbers from one number,
     * ending before another specified number - and the .rev()
     * function - which reverses the range it is called on. */
     // An inclusive Range can be made using = like so: (1..=4)
     //                   v Non-inclusive Range = (1, 2, 3)
     for number in (1..4).rev() {
        //                      ^ Reverse the Range so it is (3, 2, 1)
        println!("{number}!");
     }
     println!("LIFTOFF!!!");
}
