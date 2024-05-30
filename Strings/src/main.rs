/* The 'String' type, which is provided by Rust's standard library 
 * rather than coded into the core language, is a growable, mutable, 
 * owned, UTF-8 encoded string type. */
fn main() {
    /* Many of the same operations available with 'Vec<T>' are available 
     * with 'String' as well, because 'String' is actually implemented as 
     * a wrapper around a vector of bytes with some extra guarantees, 
     * restrictions, and capabilities. An example of a function that works 
     * the same way with 'Vec<T>' and 'String' is the '::new' function to 
     * create an instance. */
    let mut _s = String::new();
    /* This line creates a new empty string called 's', which we can then 
     * load data into. Often, we'll have some initial data that we want
     * to start the string with. For that, we use the 'to_string' method, 
     * which is available on any type that implements the 'Display' 
     * trait, as string literals do. */
    let data = "initial contents";

    let _s = data.to_string();

    // The method also works on a literal directly
    let _s = "intial contents".to_string();
    
    /* We can also use the function 'String::from' to create a 'String' 
     * from a string literal. */
    let _s = String::from("initial contents");

    /* A 'String' can grow in size and its contents can change, just 
     * like the contents of a 'Vec<T>', if you push more data into it. 
     * In addition, you can conveniently use the '+' operator or the 
     * 'format!' macro to concatenate 'String' values.
     *
     * We can grow a 'String' by using the 'push_str' method to append 
     * a string slice. */
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    /* After these two lines, 's' will contain 'foobar'. The 'push_str' 
     * method takes a string slice because we don't necessarily want to 
     * take ownership of the parameter. For example, in the code below, we
     * want to be able to use 's2' after appending its contents to 
     * 's1'. */
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    /* If the 'push_str' method took ownership of 's2', we wouldn't be 
     * able to print its value on the last line.
     *
     * The 'push' method takes a single character as a parameter and adds
     * it to the 'String'. */
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    /* String Concatenation using '+' and 'format!' */
    /* Often, you'll want to combine two existing strings. One way to 
     * do that is to use the '+' operator. */
    let s1 = String::from("Hi, ");
    let s2 = String::from("Mom!");
    let s3 = s1 + &s2;
    println!("{s3}");
    /* The string 's3' will contain 'Hi, Mom!'. The reason 's1' is no 
     * longer valid after the addition, and the reason we used a 
     * reference to 's2', has to do with the signature of the method 
     * that's called when we use the '+' operator. The '+' operator uses 
     * the 'add' method, whose signature looks something like below. */
    // fn add(self, s: &str) -> String {
    /* In the standard library, you'll see 'add' defined using generics
     * and associated types. Here, we’ve substituted in concrete types,
     * which is what happens when we call this method with String values.
     * We’ll discuss generics in Chapter 10. This signature gives us the
     * clues we need to understand the tricky bits of the '+' operator. */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    /* The 'format!' macro works similarly to 'println!', but instead 
     * of printing the output to the screen, it returns a 'String' with 
     * the contents. */
    println!("{s}");

    /* Methods for Iterating Over Strings */
    /* The best way to operate on pieces of strings is to be explicit 
     * about whether we want characters or bytes. For individual Unicode 
     * scalar values, use the 'chars' method. Calling 'chars' on "Зд" 
     * separates out and returns two values of type 'char', and you can 
     * iterate over the result to access each element. */
    /* This first loop prints each Cyrillic character as seen in the 
     * string literal being operated on. */
    for c in "Зд".chars() {
        println!("{c}");
    }
    /* This second loop prints byte values for each character 
     * because each Cyrillic character is 2 bytes wide as opposed to 
     * one. */
    for c in "Зд".bytes() {
        println!("{c}");
    }
}
