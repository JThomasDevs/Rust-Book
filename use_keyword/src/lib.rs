/* Having to write out the paths to call functions can feel inconvenient 
 * and repetitive. Instead of manually accessing a module or its members 
 * by repeatedly using an absolute or relative path, we can utilize the 
 * 'use' keyword to bring a module or module member into scope. */
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// This line brings the 'hosting' module into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Now one of the 'hosting' member functions can be called like so
    hosting::add_to_waitlist();
}
/* Paths brought into scope with 'use' also check privacy, 
 * like any other paths. */

/* Note that 'use' only creates the shortcut for the particular scope 
 * in which the 'use' occurs. So if we place the 'eat_at_restaurant' 
 * method in a sub-module, the 'hosting' module will no longer be in 
 * the correct scope to call the function in the same way. */
#[allow(dead_code)]
mod customer {
    pub fn eat_at_restaurant() {
        /* The following line, when uncommented, fails to compile 
         * because hosting has not been brought into scope here. */
        //hosting::add_to_waitlist();
    }
}

/* In the above example, we could include the function name after 
 * 'hosting' in the 'use' declaration so we could call the function just 
 * using its name. However, including the hosting name when calling the 
 * function - and thus, not inclduing the function name in the 'use' 
 * claration, is more idiomatic. Requiring the module name when calling 
 * the function makes it clear that the function is not defined 
 * locally while still minimizing repetition of the full path.
 * 
 * Conversely, when bringing in structs, enums, and other items with 
 * 'use', it's idiomatic to specify the full path. */
use std::collections::HashMap;
#[allow(dead_code)]
fn insert() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
/* There is no particular reason for this idiom. It is simply the style 
 * that has emerged from within the Rust community. */

/* The exception to this idiom is if we're bringing two items with the 
 * same name into scope with 'use' statements, because Rust doesn't 
 * allow that. */
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}
fn function2() -> io::Result<()> {
    Ok(())
}
/* As seen above, using the parent modules distinguishes the two 
 * 'Result' types. If instead we specified 'use std::fmt::Result' and 
 * 'use std::io::Result', we'd have two 'Result' types in the same scope 
 * and Rust wouldn't know which one we meant when we used 'Result'. */

/* There's another solution to the problem of bringing two types of the 
 * same name into the same scope with 'use': after the path, we can 
 * specify 'as' and a new local name, or 'alias', for the type. */
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    Ok(())
}
fn function4() -> IoResult<()> {
    Ok(())
}
/* In the second 'use' statement, we chose the new name 'IoResult' for 
 * the 'std::io::Result' type, which won't conflict with the 'Result' 
 * from 'std::fmt' that we've also brought into scope. */

/* When we bring a name into scope with the 'use' keyword, the name 
 * available in the new scope is private. To enable the code that calls 
 * our code to refer to that name as if it had been defined in that 
 * code's scope, we can combine 'pub' and 'use'. This technique is 
 * called 're-exporting' because we're bringing an item into scope but 
 * also making that item available for others to bring into their 
 * scope. */
pub use crate::front_of_house::hosting as host;

pub fn call() {
    let _ = function1();
    let _ = function2();
    let _ = function3();
    let _ = function4();
    host::add_to_waitlist();
}

/* If we want to use multiple items defined in the same crate or same 
 * module , listing each item on its own line can take up a lot of 
 * vertical space in our files.
 * Instead, we can use nested paths to bring the same items into scope 
 * in one line. We do this by specifying the common part of the path, 
 * followed by two colons, and then curly brackets around a list of the 
 * parts of the paths that differ. */
#[allow(unused_imports)]
use std::{cmp::Ordering, io as io2};
/* The above is identical to
 * 
 * use std::cmp::Ordering;
 * use std::io as io2; 
 * 
 * Below is another mutation of bringing modules into scope using this 
 * technique. */
#[allow(unused_imports)]
use std::io::{self as io3, Write};
/* Here, we are bringing 'io' into scope as well as 'Write' which is 
 * contained in 'io'. We want to bring both into scope as we may not 
 * want to include the 'io' bit of the path every time we use 'Write'. */

/* If we want to bring ALL public items defined in a path into scope, 
 * we can specify that path followed by the glob operator - '*'. */
#[allow(unused_imports)]
use std::collections::*;
/* This 'use' statement brings all public items define in 
 * 'std::collections' into the current scope. Be careful when using the 
 * glob operator! Glob can make it harder to tell what names are in 
 * scope and where a name used in your program was defined.
 * 
 * The glob operator is often used when testing to bring everything 
 * under test into 'tests' module. */