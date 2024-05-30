/* The main purpose of Ownership in Rust is to manage
 * data on the heap.
 * Ownership in Rust follows three basic rules:
 * 1) Each value in Rust has an 'owner'
 * 2) There can only be one owner at a time
 * 3) When the owner goes out of scope, the value will be dropped
 */
fn main() {
    // Explaining variable scope
    scope();

    /* Using the String type to learn about ownership */
    /* The String type manages data allocated on the heap
     * and because of this, is able to store an amount of
     * text unknown to us at compile time. Unlike string
     * literals(&str), Strings are mutable
     * You can create a 'String' from a string literal(&str)
     * using the 'from' function.
     * The :: syntax allows this from function to be
     * 'namespaced' under the 'String' type*/
    let mut s = String::from("hello");
    s.push_str(", world!"); // .push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'
    /* So what's the difference?
    *
    * string literals have contents known at compile time
    * and therefore, the contents of a &str are hardcoded
    * directly into the executable. This is only because of
    * &str's immutability. For text whose size is unknown
    * at compile time, we need the String type as we can't
    * simply put an unknown size blob of memory into the
    * binary for each piece of text whose size is unknown
    * at compile time. */
    /* The String type works by allocating an amount of memory
     * on the heap, unknown at compile time, to hold the contents.
     * This means:
     * - The memory must be requested from the memory allocator at
     *   runtime.
     * - We need a way of returning this memory to the allocator
     *   when we're done with the 'String'. */

    /* The following statements do as you'd probably expect.
     * x is assigned 5, then y is assigned the value of x,
     * so x and y are both = 5 */
    let x = 5;
    let y = x;
    println!("x is {x} and y is {y}");
    /* In the above statement, two integers are assigned.
     * Because integers are simple values with a known, fixed
     * size, they are pushed onto the stack during execution. */

    /* The below statements look similar, but when uncommented,
     * they fail to compile. */
    //let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}, world!", s1);

    /* This is because Strings are not like simple values.
     * Under the hood, a String is made of three parts:
     * - a pointer to the memory that hold the string contents
     * - a length
     * - a capacity
     * This group of data is stored on the stack.
     * The length is how much memory, in bytes, the contents
     * of the 'String' are currently using.
     * The capacity is the total amount of memory, in bytes,
     * that the 'String' has received from the allocator.
     * This difference does matter, but not in this context.
     *
     * When we assign 's1' to 's2', the String data is copied,
     * meaning we copy the point, the length, and the capacity
     * that are on the stack. We do not copy the data on the
     * heap that the pointer refers to.
     * In addition to copying the data from s1 to s2, s1 is also
     * invalidated during s2's assignment.
     * This is called a 'move' and it prevents 'double free errors'
     * which is a type of memory safety bug that occurs when the
     * same piece of memory is attempted to be freed twice as
     * would have happened if s1 AND s2 left scope with both
     * variables being still valid. Remember, s2 copied the pointer
     * from s1, not the actual String contents AND that Rust
     * automatically invalidates variables and frees their memory
     * when they leave scope.
     * If we want to make a deep copy of s1 - that is, if we want
     * to copy the String data from s1, store it in a new memory
     * location and assign a pointer to this location to s2, we
     * can use a common method called 'clone'. */
    // 'clone' example
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    let x = 5;
    let y = x; // These statements are valid
    println!("x = {}, y = {}", x, y);
    /* So why do we have to call .clone() on a String type but
     * not on the integer types used above?
     * The reason for this is that types like integers that
     * have a known size at compile time are stored entirely
     * on the stack, therefore making it quick to copy actual
     * values. This means that there is no point in invalidating
     * 'x' after 'y' is created. For values on the stack, there
     * is no difference between shallow and deep copying.
     * This works because Rust has an annotation 'Copy' that
     * can be placed on types that are stored on the stack.
     * If a type uses this 'Copy' trait, variables of that type
     * do not 'move' but are rather trivially copied, making
     * them stay valid after assignment to another variable.
     * Additionally, a type cannot be annotated with 'Copy' if
     * the type, or any of its parts, has implemented the 'Drop'
     * trait. In other words, if a type needs something special
     * to happen when the value goes out of scope, we cannot
     * use the 'Copy' trait. */

    /* Ownership with Functions */
    let s = String::from("hello");
    take_ownership(s);
    // If we try to use s here, the code will not compile
    //println!("s is: {s}");

    let x = 5;
    makes_copy(x);
    // We can still use x here, because it was copied, not moved
    println!("x is: {x}");

    { // Inner Scope
        // gives_ownership() moves its return value into s1
        let s1 = gives_ownership();
        // s2 comes into scope
        let s2 = String::from("Hi");
        /* s2 is moved into takes_and_gives_back, which also
         * moves its return value into s3 */
        let mut s3 = takes_and_gives_back(s2);
        s3.push_str(&s1);
        println!("{}", s3);
    } /* Here, s3 goes out of scope and is dropped. s2 was moved,
       * so nothing happens. s1 goes out of scope and is dropped. */

    /* The ownership of a variable follows the same pattern
     * every time: assigning a value to another variable moves
     * it. When a variable that includes data on the heap goes out of
     * scope, the value will be cleaned up by 'Drop' unless ownership
     * of the data has been moved to another variable.
     * While this works, it can be tedious taking and returning
     * ownership with every function.
     * What if we want a function to use a value but NOT take
     * ownership?
     * Rust has a feature for using a value without transferring
     * ownership caled 'references' */
}

fn scope() {
    /* Scope is the range within a program for which
     * an item is valid. */
    // Take the following variable
    let _s = "hello";
    /* This variable is valid from the point it is declared
     * until the end of the current scope - the end of the
     * function, in this case. */
    {
        // x is not valid here, it's not yet declared
        let _x = "no";

        // do stuff with x
    } // this scope is now over, and x is no longer valid

    /* When uncommented, the code fails to compile
     * as _x is not declared in this scope. */
    //let a = _x;
}

fn take_ownership(string1: String) {
    //string1 comes into scope
    println!("{}", string1);
} // Here, string1 goes out of scope and 'Drop' is called.
  // The memory used by string1 is freed on the closing bracket

fn makes_copy(some_int: i32) {
    // some_int comes into scope
    println!("{}", some_int);
} // Here, some_int goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from(", Mom!");
    /* some_string is returned and moves out to
     * the calling function */
    some_string
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}