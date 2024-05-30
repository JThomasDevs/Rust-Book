/* Vectors allow us to store more than one value in a single data 
 * structure that puts all the values next to each other in memory. 
 * Vectors can only store values of the same type. */
fn main() {
    /* To create a new empty vector, we call the 'Vec::new' function. */
    let _v: Vec<i32> = Vec::new();
    /* Note that type annotation is used for this declaration. Because 
     * we aren't inserting any values into this vector, Rust doesn't 
     * know what kind of elements we intend to store. This is an 
     * important point. Vectors are implemented using generics. For now, 
     * know that the 'Vec<T>' type provided by the standard library can 
     * hold any type. When we create a vector to hold a specific type, 
     * we can specify the type within angle brackets.
     * Above, the vector is specified to hold elements of type 'i32'.
     * 
     * If a vector is created with initial values, Rust will infer the 
     * type of value to be stored in the vector, and type annotation is 
     * not needed. Rust provides the 'vec!' macro, which will create a 
     * new vector that holds the values you give it. */
    let _v = vec![1, 2, 3];
    /* Because we've given initial 'i32' values, Rust can infer that the 
     * type of '_v' is 'Vec<i32>'. */

    /* To create a vector and then add elements to it, we can use the 
     * 'push' method. */
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    dbg!(&v);

    /* There are two ways to reference a value stored in a vector: via 
     * indexing or using the 'get' method. */
    let v = vec![String::from("yes"), String::from("no"), String::from("maybe")];

    let third = &v[2];
    println!("The third element is {third}");
    println!("{:?}", v);

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /* Iterating over the Values in a Vector */
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        /* To change the value that the mutable reference refers to, 
         * we have to use the '*' dereference operator to get to the
         * value in 'i' before we can use the '+=' operator.
         * The dereference operator is covered further in later 
         * sections. */
        *i += 50;
    }
    /* Iterating over a vector, whether immutably or mutably, is safe 
     * because of the borrow checker's rules. If we attempted to insert 
     * or remove items in the 'for' loop bodies in the example above, 
     * we would get a compiler error. The reference to the vector that 
     * the 'for' loop holds prevents simultaneous modification of the 
     * whole vector. */
    
    /* Using an Enum to Store Multiple Types */
    /* Vectors can only store values that are the same type. This can be 
     * inconvenient; there are definitely use cases for needing to store 
     * a list of items of different types. Fortunately, the variants of an 
     * enum are defined under the same enum type, we when we need one type 
     * to represent elements of different types, we can define and use an 
     * enum!
     *
     * For example, say we want to get values from a row in a spreadsheet 
     * in which some of the columns in the row contain integers, some 
     * floating-point numbers, and some strings. We can define an enum 
     * whose variants will hold the different value types, and all the 
     * enum variants will be considered the same type: that of the enum. 
     * Then we can create a vector to hold that enum and so, ultimately, 
     * hold different types. */
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    /* Rust needs to know what types will be in the vector at compile time 
     * so it knows exactly how much memory on the heap will be needed to 
     * store each element. We must also be explicit about what types are 
     * allowed in this vector. If Rust allowed a vector to hold any type, 
     * there would be a chance that one or more of the types would cause 
     * errors with the operations performed on the elements of the vector. 
     * Using an enum plus a 'match' expression means that Rust will ensure 
     * at compile time that every possible case is handled. */

    /* Dropping a Vector Drops its Elements */
    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with _v
    } // <- _v goes out of scope and is freed here
    /* When the vector gets dropped, all of its contents are also dropped, 
     * meaning the integers it holds will be cleaned up. The borrow checker 
     * ensures that any references to contents of a vector are only used 
     * while the vector itself is valid. */
}
