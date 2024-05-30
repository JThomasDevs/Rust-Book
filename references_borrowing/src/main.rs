fn main() {
    /* To calculate the length of a String, we can
     * move the String to a function that returns
     * a tuple containing the String and the length.
     * The String needs to be moved back out of the 
     * function if we intend to keep using it. */
     let s1 = String::from("hello");
     let (s2, len) = calculate_length_tup(s1);
     println!("The length of '{s2}' is {len}.");

    /* But this is a lot of extra work for such a common
     * concept. It can be called using a Rust feature known
     * as 'references' that allow using a value without
     * transferring ownership. 
     * 
     * A 'reference' is like a pointer in that it's an address
     * we can follow to access the data stored at that address;
     * that data is owned by some other variable. Unlike a
     * pointer, a reference is guaranteed to point to a valid
     * value of a particular type for the life of that reference. */
    let s1 = String::from("hello again");
    let len = calculate_length(&s1); // Using a reference to get the String length
    /* &s1 is a 'reference' to s1 and prevents s1 from being moved */
    println!("The length of '{s1}' is {len}.");

    /* The action of creating a reference is called 'borrowing'.
     * Just as in real life, if a person owns something, you
     * can borrow it from them. When you are done with it,
     * you must return it to them. You don't own it. */
    /* If we want to modify something we are borrowing,
     * we must make the reference AND the variable being
     * referenced 'mutable'. */
    let mut s = String::from("Hi");
    change(&mut s);
    println!("{s}");
    /* Note that above, s AND the argument passed to change() are
     * both marked as mutable. */
    /* Mutable references have one big restriction. If you have
     * a mutable reference to a value, you can have no other 
     * references to that value. */
    // The below statements, when uncommented, do not compile.
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}");
    /* This restriction exists to prevent data races at compile
     * time. A 'data race' is similar to a race condition and
     * happens when these three behaviors occur:
     * - Two or more pointers access the same data at the same time
     * - At least one of the pointers is being used to write to the data
     * - There's no mechanism being used to sychronize access to the 
     *   data. 
     * Data races cause undefined behavior and can be difficult
     * to track down and fix at runtime. Rust prevents this problem
     * by refusing to compile code with data races! 
     * Multiple mutable references can exist, just not in the
     * same scope. */
     let mut s = String::from("yellow");
     {
        let r1 = &mut s;
        r1.push_str(" car");
     } // r1 goes out of scope here, so we can make another reference with no problems
     let r2 = &mut s;
     r2.push_str("!");
     println!("r2: {r2}");

    // The below code also does not compile
    //  let mut some_string = String::from("punch");
    //  let r1 = &s; // no problem
    //  let r2 = &s; // no problem
    //  let r3 = &mut s; // BIG PROBLEM
    //  println!("{r1}, {r2}, and {r3}");
    /* In addition to the restriction on multiple mutable references,
     * Rust disallows having a mutable reference while we have an
     * immutable reference. This is because users of an immutable
     * reference do not expect the value to suddenly change. To the
     * contrary, multiple immutable references are allowed because
     * no one who is just reading the data has the ability to affect
     * anyone else's reading of the data. 
     * Note that a reference's scope starts from where it is first
     * introduced and continues through the last time that reference
     * is used. 
     * The following code, while similar to the statements above,
     * does compile because the immutable references are out of scope
     * by the time the mutable reference comes into scope. */
    let mut s = String::from("punch");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3} buggy!");

    /* In languages with pointers, it's easy to erroneously create a
     * 'dangling pointer' - a pointer that references a location in
     * memory while preserving a pointer to that memory. In Rust, 
     * by contrast, the compiler guarantess that references will
     * never be dangling references. If you have a reference to some
     * data, the compiler will ensure that the data will not go out
     * of scope before the reference to the data does. */
    //let reference_to_nothing = dangle();
    /* Because 's' is created inside the 'dangle' function, when the
     * code of 'dangle' is finished, 's' will be deallocated. But we
     * tried to return a reference to it. That means this reference
     * would be pointing to an invalid 'String'. Rust won't let us
     * do this. The solution here is to return the String directly: */
    let _ = no_dangle(); // No problems!
}

fn calculate_length_tup(s: String) -> (String, usize) {
    let length = s.len(); // .len() returns the length of a String

    (s, length) // Return a tuple with the String and the length
}

fn calculate_length(s: &String) -> usize {
    //                 ^ The & mean that this is a 'reference'
    s.len()
} /* s goes out of scope here. But because it does not have
   * ownership of what it refers to, it is not dropped. */

fn change(s: &mut String) {
    s.push_str(", Mom!");
}

/* THE BELOW FUNCTION WILL CAUSE A COMPILER ERROR */
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

fn no_dangle() -> String {
    let s = String::from("hello");

    s // Move the String 's' out of the function
}