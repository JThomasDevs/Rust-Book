/* Generic Data Types */
fn main() {
    /* Removing Duplication by Extracting a Function */
    /* Generics allows us to replace specific types with a placeholder
     * that represents multiple types to remove code duplication. Before
     * diving into generics syntax, then, let's first look at how to
     * remove duplication in a way that doesn't involve generic types by
     * extracting a function that replaces specific values with a
     * placeholder that represents multiple values. Then we'll apply the
     * same technique to extract a generic function! By looking at how to
     * recognize duplicated code you can extract into a function, you'll
     * start to recognize duplicated code that can use generics.
     *
     * Take the following program that finds the largest number in a
     * list. */
    let number_list = vec![34, 50, 25, 100, 65];

    let mut result = &number_list[0];

    for number in &number_list {
        if number > result {
            result = number;
        }
    }
    println!("1) The largest number is {}", result);

    /* We store a list of integers in the variable 'number_list' and place
     * a reference to the first number in the list in a variable named
     * 'result'. We then iterate through all the numbers in the list, and
     * if the current number is greater than the number stored in
     * 'result', replace the reference in that variable. However, if the
     * current number is less than or equal to the largest number seen so
     * far, the variable doesn't change, and the code moves on to the next
     * number in the list. After considering all the numbers in the list,
     * 'result' should refer to the largest number, which in this case
     * is 100.
     *
     * We've now been tasked with finding the largest number in two
     * different lists of numbers. To do so, we can choose to duplicate
     * the code above and use te same logic at two different places in
     * the program, as shown below. */
    let number_list = vec![34, 50, 25, 100, 65];
    let mut result = &number_list[0];
    for number in &number_list {
        if number > result {
            result = number;
        }
    }
    println!("2) The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut result = &number_list[0];
    for number in &number_list {
        if number > result {
            result = number;
        }
    }
    println!("3) The largest number is {}", result);
    /* Although this code works, duplicating code is tedious and error
     * prone. We also have to remember to update the code in multiple
     * places when we want to change it.
     *
     * To eliminate this duplication, we'll create an abstraction by
     * defining a function that operates on any list of integers passed in
     * a parameter. This solution makes our code clearer and lets us
     * express the concept of finding the largest number in a list
     * abstractly.
     *
     * Below, we extract the code that finds the largest number into a
     * function named 'largest'. Then we call the function to find the
     * largest to find the largest number in the two lists from above. We
     * could also use the function on any other list of 'i32' values we
     * might have in the future. */
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("4) The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("5) The largest number is {}", result);
    /* The 'largest' function has a parameter called 'list', which
     * represents any concrete slice of 'i32' values we might pass into
     * the function. As a result, when we call the function, the code runs
     * on the specific values that we pass in.
     *
     * Next, we'll use these same steps with generics to reduce code
     * duplication. In the same way that the function body can operate on
     * an abstract 'list' instead of specific values, generics allow code
     * to operate on abstract types.
     *
     * For example, say we had two functions: one that finds the largest
     * item in a slice of 'i32' values and one that finds the largest
     * item in a slice of 'char' values. Generics can help eliminate this
     * exact sort of code duplication so we only need one function. */

    /* We use generics to create definitions for items like function
     * signatures or structs, which we can then use with many different
     * concrete data types. Let's first look at how to define functions,
     * structs, enums, and methods using generics. Then we'll discuss how
     * generics affect code performance. */

    /* In Function Definitions */
    /* When defining a function that uses generics, we place the generics
     * in the signature of the function where we would usually specify the
     * data types of the parameters and return value. Doing so makes our
     * code more flexible and provides more functionality to callers of
     * our function while preventing code duplication.
     *
     * Continuing with the 'largest' function, the below example shows two
     * functions that both find the largest value in a slice. We'll then
     * combine these into a single function that uses generics. */
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("\nThe largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    /* The 'largest_i32' function is the one we extracted above that finds
     * the largest 'i32' in a slice. The 'largest_char' function finds the
     * largest 'char' in a slice. The function bodies have the same code,
     * so let's eliminate the duplication by introducing a generic type
     * parameter in a single function.
     *
     * To parameterize the tpes in a new single function, we need to name
     * the type parameter, just as we do for the value parameters to a
     * function. You can use any identifier as a type parameter name. But
     * we'll use 'T' because, by convention, type parameter names in Rust
     * are short, often just a letter, and Rust's type-naming convention
     * is UpperCamelCase. Short for "type", 'T' is the default choice of
     * most Rust programmers.
     *
     * When we use a parameter in the body of the function, we have to
     * declare the parameter name in the signature so the compiler knows
     * what that name means. Similarly, when we use a type parameter name
     * in a function signature, we have to declare the type parameter name
     * before we use it. To define the generic 'largest' function, place
     * type name declarations inside angle brackets, '< >', between the
     * name of the function and the parameter list, as shown below */
    {
        fn largest<T>(list: &[T]) -> &T {
            &list[0]
        }
        largest(&[1]);
    }
    /* We read this definition as: the function 'largest' is generic over
     * some type 'T'. This function has one parameter named 'list', which
     * is a slice of values of type 'T'. The 'largest' function will return
     * a reference to a value of the same type 'T'.
     *
     * The below example shows the combined 'largest' function definition
     * using the generic data type in its signature. The listing also shows
     * how we can call the function with either a slice of 'i32' values or
     * 'char' values. */
    {
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    /* The 'std::cmp::PartialOrd' bit in the function signature after 'T'
     * is called a 'trait' which will be covered in the next section. For
     * now, know that this means that the body of 'largest' won't work for
     * all possible types that 'T' could be. Because we want to compare
     * values of type 'T' in the body, we can only use types whose values
     * can be ordered. To enable comparisons, the standard library has the
     * 'std::cmp::PartialOrd' trait that can be implemented on types. By
     * using this trait, we restrict the types valid for 'T' to only those
     * that implement 'PartialOrd' and this example will compile, because
     * the standard library implements 'PartialOrd' on both 'i32' and
     * 'char'. */

    /* In Struct Definitions */
    /* We can also define structs to use a generic type parameter in one or
     * more fields using the '< >' syntax. The below example defines a
     * 'Point<T>' struct to hold 'x' and 'y' coordinate values of any
     * type. */
    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    /* The syntax for using generics in struct definitions is similar to
     * that used in function defintitions. First, we declare the name of
     * the type parameter inside angle brackets just after the name of the
     * struct. Then we use the generic type in the struct definition where
     * we would otherwise specify concrete data types.
     *
     * Note that because we've used only one generic type to define
     * 'Point<T>', this definition says that the 'Point<T>' struct is
     * generic over some type 'T', and the fields 'x' and 'y' are BOTH that
     * same type, whatever that type may be. If we create an instance of a
     * 'Point<T>' that has values of different types, as below, the code
     * won't compile. */
    {
        //let bad_instance = Point { x: 5, y: 4.0 };
    }
    /* To define a 'Point' struct where 'x' and 'y' are both generics but
     * could have different types, we can use multiple generic type
     * parameters. For example, in the below example, we change the
     * definition of 'Point' to be generic over types 'T' and 'U' where 'x'
     * is of type 'T' and 'y' is of type 'U'. */
    {
        #[allow(dead_code)]
        struct Point<T, U> {
            x: T,
            y: U,
        }
        let _both_integer = Point { x: 5, y: 10 };
        let _both_float = Point { x: 1.0, y: 4.0 };
        let _integer_and_float = Point { x: 5, y: 4.0 };
    }
    /* Now all the instances of 'Point' shown are allowed! You can use as
     * many generic type parameters in a definition as you want, but using
     * more than a few makes you code hard to read. If you're finding you
     * need lots of generic types in your code, it could indicate that your
     * code needs restructuring into smaller pieces. */

    // TODO: Continue @ Section 10.1 - In Enum Definitions
}
