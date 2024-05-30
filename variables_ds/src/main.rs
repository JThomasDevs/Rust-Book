fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const variables are ALWAYS immutable and type MUST be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds is {THREE_HOURS_IN_SECONDS} seconds");

    shadow();

    println!("const Y is: {Y}");
    
    // Booleans
    let t = true;
    println!("t is: {t}");

    let f: bool = false;
    println!("f is: {f}");

    // Tuples
    /* A tuple is a collection of comma separated values
     * in parentheses that may vary in type. */
    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");

    // Tuples can be unpacked into individual variables
    let (_w, _x, _y, z) = tup; // Not sure why the underscores are there? Try removing them and running again for a cool compiler quirk!
    println!("tuple z is: {z}");

    // Individual values within a tuple can be accessed using dot notation
    let i_32 = tup.0;
    let f_64 = tup.1;
    let u_8 = tup.2;
    let str = tup.3;
    println!("tup is: ({}, {}, {}, {})", i_32, f_64, u_8, str);


    // Arrays
    /* An array is a collection of comma seperated values
     * in square brackets. All values in an array must be
     * of the same type. 
     * Arrays are also fixed size. For an array-like container
     * of variable size, use a vector.*/
    let _arr = [1, 2, 3, 4, 5];

    // Arrays are more useful when you know the number of elements will not change
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    /* An array's type is written using square brackets with the
     * type of each element, a semicolon, and the number of 
     * elements in the array. */
    //             v number of elements in arr
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    //         ^ type of elements in arr

    /* An array can also be initialized to contain the same value
    * n times where n is the number of desired elements. */
    //            v number of elements in arr
    let arr = [3; 5];
    //         ^ element to be inserted

    // Individual array elements may be accessed using bracket notation
    println!("arr is: [{}, {}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3], arr[4]);

    // When uncommented, the following expression will panic
    //let _a: i32 = arr[10];
    /* This expression panics because the index attempting to
     * be accessed is out of the bounds of the array.
     * With 5 elements, the max index of arr is [4]. */
}

// const variables can be declared in the global scope,
// outside of any functions
const Y: &str = "1";

fn shadow() {
    let x = 5; 
    /* 
     * When x is reassigned using the 'let' keyword, the new x is said
     * to 'shadow' the previous x. Shadowing allows a single variable
     * name to be assigned multiple values or types throughout its 
     * scope. This is unlike the 'mut' keyword as 'mut' allows the value
     * of a variable only to be changed while shadowing allows for a 
     * variable's type to be changed.
     * 
     * Example:
     * let x = "17"; <-- &str
     * let x: i32 = string.parse().expect(""); <--i32
     * assert_eq!(x, 17); <-- True
     * 
     * In the above example, the number 17 is parsed from the previous
     * assigned value of x and the type of x is changed to u32 and
     * assigned the u32 value 17. After this re-assignment using let, 
     * x - which is now a u32 - is still immutable.
     */
    let x = x + 1; // This x 'shadows' the first declared instance of x

    { // Inner Scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // Back to function scope
    println!("The value of x is: {x}");

    // Change a variable's type using shadowing
    let spaces = "      "; // Type: &str
    println!("spaces variable type: {}", typee(&spaces));
    let spaces = spaces.len(); // Type: usize
    println!("spaces variable type after shadow: {}", typee(&spaces));
    let spaces: u32 = spaces.try_into().expect(""); // Type: u32
    println!("spaces variable type after shadow2: {}", typee(&spaces));
    /* 
     * If we had used:
     * 
     * let mut spaces = "      ";
     * spaces = spaces.len();
     * 
     * the compiler would give us a mismatched types error
     * as we would be attempting to assign a non &str value
     * which the variable currently holds. Use 'let' keyword
     * paired with type-specific functions to change a variable's 
     * type.
     */
}

/// This function returns the type of the passed variable as &str
pub fn typee<T>(var: &T) -> &str {
    std::any::type_name_of_val(var)
}