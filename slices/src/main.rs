/* 'Slices' let you reference a contguous sequence of elements in a
 * collection rather than the whole collection. A slice is a kind
 * of reference, so it does not have ownership.
 *
 * Say we want to write a function that takes a string of words
 * separated by spaces and returns the first word it find in that
 * string. If the function doesn't find a space in the string,
 * the whole string must be one word, so the entire string should
 * be returned.
 *
 * A good way to do this is to use String slices: */
fn main() {
    /* A string slice is a reference to part of a String,
     * and it looks like this: */
    let s = String::from("hi mom");

    let _hi = &s[0..2]; // non-inclusive index range
    let _mom = &s[3..6]; // "mom"

    /* Rather than a reference to the entire String, _hi is a
     * reference to a portion of the String, specified in the
     * extra [0..2] bit. Slices are created using a range within
     * brackets by specifying '[start_index..end_index]', where
     * 'start_index' is the first position in the slice and
     * 'end_index' is one more than the last position in the slice.
     * Internally, the slice data structure stores the starting
     * position and the length of the slice, which corresponds to
     * 'end_index' minus 'start_index'.
     * With Rust's '..' range syntax, if you want to start at
     * index 0, you can drop the value before the two periods. */
    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];

    /* In the same fashion, if your slice includes the last byte
     * of the 'String', you can drop the trailing number. */
    let s1 = String::from("hello");
    let len = s.len();
    let _slice = &s1[0..len];
    let _slice = &s1[..];

    /* With all this in mind, let's write the function described
     * at the start of the file. */
    let s2 = String::from("Hi Mom!");
    let _hi = first_word(&s2);
    //s2.clear(); // this line cause a compiler error when uncommented
    println!("the first word is: {_hi}");

    /* Recall that string literals are stored inside the binary.
     * Now that we know about slices, we can properly understand
     * string literals. */
    let _s = "Hello, world!";
    /* The type of 's' here is '&str': it's a slice pointing to 
     * that specific point of the binary. This is also why
     * string literals are immutable; '&str' is an immutable 
     * reference. */

    /* Changing our function signature to take a string slice
    * instead of a reference to a String makes our API more
    * general without sacrificing any functionality. */
    let my_string = String::from("hello world");

    // first_word works on slices of 'String's, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);

    let my_literal = "hello world";
    // first_word also works on slices of string literals, whether partial or whole
    let _word = first_word(&my_literal[0..6]);
    let _word = first_word(&my_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_literal);

    /* OTHER TYPES OF SLICES */
    let a = [1, 2, 3, 4, 5];
    /* Just as we might want to refer to part of a string, we might
     * want to refer to part of an array. */
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    /* The slice has the type '&[i32]'. It works the same way as
     * string slices do, by storing a reference to the first 
     * element and a length. */
}

/* This version of the function's signature allows us to
 * use the same function on both '&String' and '&str' values. 
 * In other words, if we have a 'String', we can pass a slice
 * of the String OR a reference to the String. This flexibility
 * takes advantage of 'deref coercions', a feature covered later. */
fn first_word(s: &str) -> &str {
//fn first_word(s: &String) -> &str {
    // Convert s to an array of byte references
    let bytes = s.as_bytes();

    // Iterate over the array of bytes
    for (i, item) in bytes.iter().enumerate() {
        // v dereference the current byte to convert &u8 to u8
        if *item == b' ' {
            // If the current byte is a space
            return &s[..i]; // Return the slice from 0 to i
        }
    }
    // If we get here, no spaces were found, return the whole string as a slice
    &s[..]
}
