#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    /* The 'if let' syntax lets you comine 'if' and 'let' into a less 
     * verbose way to handle values that matchone pattern while ignoring 
     * the rest. */
    // Verbose using 'let' and 'match'
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    /* Using the traditional match statement, we have a special output 
     * for a Some value and we want nothing to happen for any other 
     * value. To satisfy the 'match' expression's need for all patterns 
     * to be covered, we add '_ => ()' after processing just one 
     * variant, which is annoying boilerplate code to add.
     * 
     * Instead, we can use the 'if let' syntax. */
    let config_max = Some(3u8);
    //                    v Pattern to be matched against
    if let Some(max) = config_max {
        //    ^ The matching pattern
        println!("The maximum is configured to be {}", max);
    }
    /* The 'if let' takes a pattern and an expression separated by an 
     * equal sign. It works the same way as a 'match', where the 
     * expression is given to the 'match' and the pattern is its first 
     * arm. In this case, the pattern is 'Some(max)', and the 'max' 
     * binds to the value inside the 'Some'. We can then use 'max' in 
     * the body of the 'if let' block in the same way we used 'max' in 
     * the corresponding 'match' arm. The code in the 'if let' block 
     * isn't run if the value doesn't match the pattern.
     * 
     * We can include an 'else' with an 'if let'. The block of code that 
     * goes with the 'else' is the same as the block of code that would 
     * go with the '_' case in the 'match' expression that is equivalent 
     * to the 'if let' and 'else'. */
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("{}", count);
}
