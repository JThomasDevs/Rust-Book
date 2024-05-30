/* The 'match' control flow construct is extremely powerful, allowing 
 * us to compare a value against a series of patterns and then execute 
 * code based on which pattern matches the value. Patterns can be made 
 * ip of literal values, variable names, wildcards, and many other 
 * things. The power of 'match' comes from the expressiveness of the 
 * patterns and the fact that the compiler confirms that all possible 
 * cases are handled.
 * 
 * A 'match' expression as being like a coin-sorting machine: coins 
 * slide down a track with variously sized holes along it, and each coin 
 * falls through the first hole it encounters that it fits into. In the 
 * same way, values go through each pattern in a 'match', and at the 
 * first pattern the value "fits", the value falls into the associated 
 * code block to be used during execution. */
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        /* The code associated with each arm is an expression, and the 
         * resultant value of the expression in the matching arm is the value 
         * that gets returned for the entire 'match' expression. */
        /* Curly brackets are only necessary in a branch expression if 
         * we want to execute multiple lines of code before we return
         * a value. */
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // No comma needed as the closing bracket marks the expression end
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{value}");

    //                 vv Value to be matched against
    value_in_cents(Coin::Quarter(UsState::Alaska));
    //                                  ^^ 'state' value passed to the expression in the Quarter branch

    /* In a similar fashion, we can operate on the value contained in an 
     * Option<T> using a match branch. */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // If None, we do nothing
            None => None,
            // If Some, add one to the value and return as an Option<i32> - extracting the value
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}\nnone is {:?}", six, none);

    /* Knowing that the 'Option<T>' type is an enum, knowing that we can use
     * a match statement to execute arbitrary code based on an enum value, 
     * and knowing that we can extract and use values from an enum using 
     * these expressions, we can probably guess how to convert an 
     * 'Option<T>' to a 'T'. */
    let an_option = Some(5);
    let value = match an_option {
        Some(i) => i,
        None => -1,
    };
    println!("{}", value+1);

    /* 'match' arm patterns MUST cover all possibilities.
     * When uncommented, the below code will not compile because the 'match' 
     * arms do not cover all possibilities (None is missing). */
    // fn plus_two(x: Option<i32>) -> Option<i32> {
    //    match x {
    //        Some(i) => Some(i + 2),
    //    }
    // }

    /* Using enums, we can also take special actions for a few particular 
     * values, but for all other values take one default action. */
    let dice_roll: u8 = 15;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => move_player(other),
    }

    fn add_hat() {}
    fn remove_hat() {}
    fn move_player(_num_spaces: u8) {}
    /* The first two arms have patterns that are literal values '3' and '7'. 
     * For the last arm that covers every possible value, the pattern is the 
     * variable we've chosen to name 'other'. The code that runs for the 
     * 'other' arm uses the variable by passing it to the 'move_player' 
     * function.
     * 
     * This code compiles, even though we have not listed all possible values 
     * a 'u8' can have, because the last pattern will match all values not 
     * specifically listed. This catch-all pattern meets the requirement that 
     * 'match' must be exhaustive.
     * 
     * If we have a match statement with a catch-all pattern but we don't 
     * want to use the value in the catch-all pattern, we can use '_'.
     * '_' is a special pattern that matches any value and does not bind to 
     * that value. This tells Rust we aren't going to use the value, so Rust 
     * won't warn us about an unused variable. */
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    /* Finally, if we want a catch-all arm that does nothing, so we can 
     * have a match statement that only does anything on specific patterns, 
     * we can use the 'unit value' (an empty tuple) as the code that goes 
     * with the '_' arm. */
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => (),
    }
}