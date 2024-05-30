/* 'Structs' or 'structures' are similar to tuples in that
 * they hold multiple related values that may be of varying
 * types. Unlike with tuples, values in a 'struct' must each
 * have a name so it's clear what the data represents. Naming
 * values provides another advantage over tuples: we need not
 * rely on the order of the data to specify or access the
 * values of an instance. */
/* To define a struct, we use the 'struct' keyword and name
 * the entire struct. A struct's name should describe the
 * significance of the pieces of data being grouped together.
 * Then, inside the curly brackets, we define the names and
 * types of the piece of data, which we call 'fields'. */

//       v Name of the struct
struct User {
    //       v type of the field
    active: bool,
    // ^ name of the field
    username: String,
    email: String,
    sign_in_count: u64,
}
// TODO: Continue @ Ownership of Struct Data - Section 5.1
/* Ownership of Struct Data
 *
 * In the User struct defined above, we used the owned 'String' type
 * rather than the '&str' string slice type. This is a deliberate choice
 * because we want each instance of this struct to own all of its data
 * and for that data to be valid for as long as the entire struct is
 * valid.
 * It's also possible for structs to store references to data owned by
 * something else, but to do so requires the use of 'lifetimes', a Rust
 * feature that will be covered later. Lifetimes ensure that the data
 * referenced by a struct is valid for as long as the struct is. Let's say
 * you try to store a reference in a struct without specifying lifetimes,
 * like the following; this won't work: */
// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
/* When uncommented, the above struct causes a compiler error. */

fn main() {
    /* To use a struct after we've defined it, we create an
     * instance of that struct by stating the name of the struct
     * followed by curly brackets containing key:value pairs where
     * the keys are the names of the fields and the values are the
     * data we want to store in those fields.
     * We don't need to provide the fields in the declared order
     * when instantiating a struct. */
    let _user1 = User {
        // Valid instance declaration
        active: true,
        username: String::from("somename123"),
        email: String::from("someone@example.org"),
        sign_in_count: 1,
    };
    let _user2 = User {
        // Also valid instance declaration``
        sign_in_count: 3,
        email: String::from("someone_else@example.org"),
        active: false,
        username: String::from("anothername321"),
    };
    /* To get a specific value from a struct, we use dot notation. For
     * example, to access this user's email address, we use user1.email.
     * If the instance is mutable, we can change a value by using the
     * dot notation and assigning into a particular field. */
    let mut user3 = User {
        active: true,
        username: String::from("yetanotherusername"),
        email: String::from("yetanotheremail@example.org"),
        sign_in_count: 1,
    };

    let email = user3.email;
    println!("email: {email}");
    user3.email = String::from("adifferentemail@example.org");
    let email = user3.email;
    println!("email now: {email}");
    /* Note that the entire instance of the struct must be mutable; Rust
     * doesn't allow us to mark only certain fields as mutable. As with
     * any expression, we can contruct a new instance of the struct as
     * the last expression in the function body to implicitly return
     * that new instance. */
    /* The below function returns a User instance with the provided
     * username and email. */
    let _ = build_user_verbose(String::from("email@email.com"), String::from("a_username"));
    /* The new, improved function */
    let u1 = build_user(String::from("email2@email.com"), String::from("a_username"));

    /* It's often useful to create a new instance of a struct that
     * includes most of the values from another instance, but changes
     * some. This is done using 'struct update syntax'. */
    // Without using the syntax
    let u2 = User {
        active: u1.active,
        username: u1.username,
        email: String::from("uniqueemail@example.org"),
        sign_in_count: u1.sign_in_count,
    };
    // With the syntax
    let _u3 = User {
        email: String::from("uniqueemail2@example.org"),
        ..u2
    };
    /* With using the 'struct update syntax', we need only specify the
     * changed fields - email, in this case - while the '..u2' assigns
     * the values from u2 to their corresponding fields in u3.
     *
     * Note that the 'struct update' syntax uses '=' like an assignment.
     * This is because it moves the data. In the above example, we can
     * no longer use u2 as a whole after creating u3 because the String
     * in the username field of u2 was moved into u3. If we had given
     * u3 new String values for both 'email' and 'username', and thus
     * only used the 'active' and 'sign_in_count' values from u2, then
     * u2 would still be valid after creating u3. Both 'active' and
     * 'sign_in_count' are types that implement the 'Copy' trait, so
     * u2 would not be invalidated by ownership rules. */

    /* Rust also supports structs that look similar to tuples,
     * called 'tuple structs'. Tuple structs have the added
     * meaning the struct name provides but don't have names
     * associated with their fields; rather they just have the
     * types of the fields. Tuple structs are useful when you
     * want to give the whole tuple a name and make the tuple
     * a different type from other tuples, and when naming each
     * field as in a regular struct would be verbose or redundant. */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}. {}, {})", origin.0, origin.1, origin.2);
    /* Note that _black and _origin are different types because they're
     * instances of different tuple structs. Each struct defined is its
     * own type, even though the fields within the structs might have 
     * the same types. For example, a function that takes a parameter of
     * type 'Color' cannot take a 'Point' as an argment, even though
     * both types are made up of three 'i32' values. Otherwise, tuple
     * struct instances are similar to tuples in that you can 
     * destructure them into their individual pieces, and you can use a
     * '.' followed by the index to access an individual value. */

    /* We can also define structs that don't have any fields! These 
     * are called 'unit-like structs' because they behave similarly 
     * to '()', the unit type. Unit-like structs can be useful when 
     * you need to implement a trait on some type but don't have any
     * data that you want to store in the type itself. */
    // Unit-like struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    /* To define 'AlwaysEqual', we use the 'struct' keyword, the name
     * we want, and then a semicolon. No need for curly brackets or
     * parentheses! Then we can get an instance of 'AlwaysEqual' in the
     * 'subject' variable in a similar way: using the name we defined,
     * without any curly brackets or parentheses. Imagine that later,
     * we'll implement behavior for this type, such that every instance
     * of 'AlwaysEqual' is always equal to every instance of any other
     * type, perhaps to have a known result for testing purposes. We
     * wouldn't need any data to implement that behavior! How to define
     * traits and implement them on any type, including unit-like 
     * structs. */
}

fn build_user_verbose(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
/* This function works fine but with the parameter names being the
 * same as the struct names, typing all of this out can be tedious.
 * To save some time and effort, because the parameter names and the
 * struct field names are exactly the same, we can use the
 * 'field init shorthand' syntax to rewrite the above function so it
 * behaves exactly the same, but doesn't have the repetition of
 * 'username' and 'email'. */
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // sets username field to username argument
        email,    // sets email field to email argument
        sign_in_count: 1,
    }
}
