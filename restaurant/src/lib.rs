#[allow(dead_code)]
mod _front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {} 
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
/* A module is defined with the 'mod' keyword followed by the name of the 
 * module. The body of the module goes inside curly brackets.
 * Inside modules, we can place other modules, as in this case with the 
 * modules 'hosting' and 'serving'. Modules can also hold definitions for 
 * other items, such as structs, enums, constants, traits, and functions.
 * 
 * By using modules, we can group related definitions together and name 
 * why they're related. Programmers using this code can navigate the code 
 * based on the groups rather than having to read through all the 
 * definitions, making it easier to find the definitions relevant to 
 * them. Programmers adding new functionality to this code would know 
 * where to place the code to keep the program organized. 
 * 
 * This file 'lib.rs' is the 'crate root' of this 'crate'. A 'crate' is 
 * the smallest amount of code considered by the compiler at build time. 
 * The 'crate root' is the entry point for the Rust program. 'lib.rs' 
 * indicates that this is a library crate, and thus does not need a 
 * main() function. A binary crate has a crate root named 'main.rs'. The 
 * contents of either of these two files form a module named 'crate' at 
 * the root of the crate's module structure, known as the 'module tree'.
 * 
 * crate
 *  |- front_of_house
 *      |- hosting
 *      |   |- add_to_waitlist
 *      |   |- seat_at_table
 *      |
 *      |- serving
 *          |- take_order
 *          |- serve_order
 *          |- take_payment
 *
 * This tree shows how some of the modules nest inside one another. The 
 * tree also shows that some modules are 'siblings' to each other, 
 * meaning they're defined in the same module - 'hosting' and 'serving' 
 * are siblings defined within the 'front_of_house' module.
 * If module A is contained inside module B, we say that module A is 
 * the 'child' of module B and that module B is the 'parent' of module 
 * A. Note that the entire module tree is rooted under the implicit 
 * module named 'crate'. */
 mod front_of_house {
    /* Adding 'pub' to a module makes it public so that code in its 
     * ancestor modules can refer to it, but does not allow access to its 
     * inner code. Modules are simply containers. Therefore, if we wish 
     * to expose the contents of a module with other code, we must mark 
     * each item we wish to make public with the 'pub' keyword. */
    pub mod hosting {
        /* Marking the 'add_to_waitlist' function as public allows our 
         * 'eat_at_restaurant' function below to call it using its 
         * path. */
        pub fn add_to_waitlist() {}
    }
}

pub fn _eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    /* The path begins with 'front_of_house' here because our function is 
     * defined in the same module as 'front_of_house'. Therefore, our 
     * function and the front_of_house module are siblings. 
     * This is also the reason why the 'eat_at_restaurant' function can 
     * reference the front_of_house module, despite it not being 
     * public. */
}

fn deliver_order() {}
#[allow(dead_code)]
mod _back_of_house {
    /* The 'fix_incorrect_order' function is in the 'back_of_house' 
     * module, so we can use 'super' to go to the parent module of 
     * 'back_of_house', which in this case is 'crate', the root. From 
     * there, we look for 'deliver_order' and find it. */
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

/* Further, we can use 'pub' to designate structs and enums as public, 
 * but there are a few extra details to the usage of 'pub' with structs 
 * and enums. If we use 'pub' before a struct definition, we make the 
 * struct public, but the struct's fields will still be private. We can 
 * make each field public or not on a case-by-case basis. */
#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
/* The above 'back_of_house' module and 'eat_at_restaurant' function 
 * model a case in a restaurant where the customer can pick the type of 
 * bread that comes with a meal, but the chef decides which fruit 
 * accompanies the meal based on what's in season and in stock. */
/* Because the 'toast' field in the 'back_of_house::Breakfast' struct is 
 * public, in 'eat_at_restaurant' we can write and read to the 'toast' 
 * field using dot notation. Notice that we can't use the 
 * 'seasonal_fruit' field in 'eat_at_restaurant' because 'seasonal_fruit' 
 * is private.
 * 
 * Also, note that because 'back_of_house::Breakfast' has a private 
 * field, the struct needs to provicde a public associated function that 
 * constructs an instance of 'Breakfast' (we've named it 'summer' here). 
 * If 'Breakfast' didn't have such a function, we couldn't create an 
 * instance of 'Breakfast' in 'eat_at_restaurant' because we couldn't set 
 * the value of the private 'seasonal_fruit' field in 
 * 'eat_at_restaurant'. 
 * 
 * In contrast, if we make an enum public, all of its variants are then 
 * public. We only need the 'pub' keyword before the 'enum' keyword. */
mod another_back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_again() {
    let _order1 = another_back_of_house::Appetizer::Soup;
    let _order2 = another_back_of_house::Appetizer::Salad;
}