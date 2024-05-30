/* When modules get large, we might want to move their definitions 
 * to a separate file to make the code easier to navigate. */
mod front_of_house; // Module definition moved to 'front_of_house.rs'
/* Note that we only need to load a file using a 'mod' declaration ONCE 
 * in the module tree. Once the compiler knows the file is part of the 
 * project (and knows where in the module tree the code resides because 
 * of where we've put the 'mod' statement), other files in the project 
 * should refer to the loaded file's code using a path to where it was 
 * declared. In other words, 'mod' is NOT an "include" operation as 
 * found in C++. */

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}