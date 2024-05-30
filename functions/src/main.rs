// Rust supports C++ style comments
/* That means
 * block comments too! */
fn main() {
    println!("Hello, world!");

    /* Note that the below function is called before it is
     * defined in this file.
     * Functions in Rust can be called anywhere within the
     * scope in which they are defined. */
    // Call a function with its name followed by parentheses
    another_function();

    /* When calling a function with parameters, provide an
     * argument to the function call matching the parameter
     * type. */
     function_three(4, 'b');

     // Some functions return values
     let val = function_return();
     println!("val is: {val}");

     // This functions takes arguments AND returns a value
     add_nums(4, 3);
}

// Functions are defined using the 'fn' keyword
fn another_function() {
    println!("Another function.");
}

/* Functions can be defined to take parameters.
 * In the function signature(definition), all parameters
 * MUST have their types defined. */
fn function_three(x: i32, y: char) {
    println!("The value of x and y is: {x} and {y}");
}

//                       v The type of value to be returned from this function
fn function_return() -> i32 {
    /* To return a value from a function or expression, remove the 
     * semicolon at the end. This function returns 5 as an i32. */
    5
}

/* This function takes 'x' and 'y' as parameters
 * and returns their sum. */
fn add_nums(x: i32, y: i32) -> i32 {
    println!("{x} + {y} = {}", (x+y));
    x + y
}