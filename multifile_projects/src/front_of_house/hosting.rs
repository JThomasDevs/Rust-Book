pub fn add_to_waitlist() {}
/* The 'hosting' module definition in this file has been placed in a 
 * 'front_of_house' subdirectory inside the 'src' TLD. This is because 
 * the hosting module is declared in the 'front_of_house' module.
 * If we instead put 'hosting.rs' in the 'src' directory, the compiler 
 * would expect the 'hosting.rs' code to be in a 'hosting' module 
 * declared in the crate root, and not declared as a child of the 
 * 'front_of_house' module. The compiler's rules for which files to 
 * check for which modules' code means the directories and files more 
 * closely match the module tree. */