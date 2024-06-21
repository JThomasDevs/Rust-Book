use rand::Rng;
use std::cmp::Ordering;
use std::io;

/* Creating Custom Types for Validation */
fn main() {
    guessing_game();
    /* Let’s take the idea of using Rust’s type system to ensure we have a
     * valid value one step further and look at creating a custom type for
     * validation. Recall the guessing game built earlier in which our code
     * asked the user to guess a number between 1 and 100. We never
     * validated that the user’s guess was between those numbers before
     * checking it against our secret number; we only validated that the
     * guess was positive. In this case, the consequences were not very
     * dire: our output of “Too high” or “Too low” would still be correct.
     * But it would be a useful enhancement to guide the user toward valid
     * guesses and have different behavior when a user guesses a number
     * that’s out of range versus when a user types, for example, letters
     * instead. */
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /* One way to do this would be to parse the guess as an 'i32'
         * instead of only a 'u32' to allows potentially negative numbers,
         * and then add a check for the number being in range. */
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        /* The 'if' expression checks whether our value is out of range,
         * tells the user about the problem, and calls 'continue' to start
         * the next iteration of the loop and ask for another guess. After
         * the 'if' expression, we can proceed with the comparisons between
         * 'guess' and the secret number knowing that 'guess' is between
         * 1 and 100.
         *
         * However, this is not an ideal solution: if it was absolutely
         * critical that the program only operated on values between 1 and
         * 100, and it had many functions with this requirement, having a
         * check like above in every function would be tedious (and might
         * impact performance). - Jump to struct Guess definition */

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/* Instead, we can make a new type and put the validations in a
 * function to create an instance of the type rather than repeating
 * the validations everywhere. That way, it's safe for functions to
 * use the new type in their signatures and confidently use the
 * values they receive. The below code will show one way to define
 * a 'Guess' type that will only create an instance of 'Guess' if
 * the 'new' function receives a value between 1 and 100. */
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Result<Guess, io::Error> {
        if value < 1 || value > 100 {
            return Err(io::Error::other("Guess must be between 1 and 100."));
        }
        return Ok(Guess { value });
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
/* First we define a struct named 'Guess' that has a field named 'value'
 * that holds an 'i32'. This is where the number will be stored.
 *
 * Then we implement an associated function named 'new' on 'Guess' that
 * creates instances of 'Guess' values. The 'new' function is defined
 * to have on parameter named 'value' of type 'i32' and to return a
 * 'Guess'. The code in the body of the 'new' function tests 'value' to
 * make sure it's between 1 and 100. If 'value' doesn't pass this test,
 * we make a 'panic!' call, which will alert the programmer who is
 * writing the calling code that they have a bug they need to fix,
 * because creating a 'Guess' with a 'value' outside this range would
 * violate the contract that 'Guess::new' is relying on. The conditions
 * in which 'Guess::new' might panic should be discussed in its public-
 * facing API documentation. The specifics of documentation conventions
 * are covered later. If 'value' does pass the test, we create a new
 * 'Guess' with its 'value' field set to the value parameter and return
 * the 'Guess'.
 *
 * Next, we implement a method named 'value' that borrows 'self',
 * doesn't have any other parameters, and returns an 'i32'. This kind
 * of method is sometimes called a 'getter', because its purpose is to
 * get some data from its fields and return it. This public method is
 * necessary because the 'value' field of the 'Guess' struct is private.
 * It's important that the 'value' field be private so code using the
 * 'Guess' struct is not allowed to set value directly: code outside the
 * module MUST use the 'Guess::new' function to create and instance of
 * 'Guess', thereby ensuring there's no way for a 'Guess' to have a
 * 'value' that hasn't been checked by the conditions in the
 * 'Guess::new' function.
 *
 * A function that has a parameter or returns only numbers between 1 and
 * 100 could then declare in its signature that it takes or returns a
 * 'Guess' rather than an 'i32' and wouldn't need to do any additional
 * checks in its body. */
fn guessing_game() {
    println!("Guess the number! V2");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
            Err(_) => continue,
        };

        println!("You guessed {}.", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
