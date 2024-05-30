mod tests;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/* Instead of creating a completely separate function to calculate the
 * area of a rectangle, we can implement a method on the 'Rectangle'
 * type to do the same thing.
 * Methods are similar to functions in that they are declared with the
 * 'fn' keyword and a name, they can have parameters and a return value,
 * and they contains some code that's run when the method is called from
 * somewhere else. Unlike functions, methods are defined within the
 * context of a struct (or an enum or a trait object), and their first
 * parameter is always 'self', which represents the instance of the
 * struct the method is being called on. */
/* Implement a method on a struct using the 'impl' keyword and the
 * struct's name. */
impl Rectangle {
    //      v Note that self is borrowed in the area() method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    /* As mentioned above, to define the area function in the context of
     * the 'Rectangle' struct, we use 'impl' to start an implementation
     * block. Everything within this 'impl' block will be associated with
     * the 'Rectangle' type. Then we move the 'area' function within the
     * 'impl' curly brackets and change the first (and only in
     * this case) parameter to be 'self' in the signature and everywhere
     * within the method body. As opposed to passing the Rectangle
     * instance to a function, method syntax allows us to call the
     * 'area' method on our 'Rectangle' instance. Calling a method on an
     * instance of a type uses dot notation. (e.g., rect1.area()) */

    /* Methods can work with any amount of parameters after 'self'
     * parameter, and those parameters work just like parameters in
     * functions. */
    // Define a method that returns if one rectangle fits entirely inside another
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    /* All functions defined within an 'impl' block are called
     * 'associated functions' because they're associated with the type
     * named after the 'impl'. We can define associated functions that
     * don't have 'self' as their first parameter(and thus, are not
     * methods) because they don't need an instance of the type to work
     * with. An example of such an associated function is 'String::from'
     * function that's defined on the 'String' type.
     *
     * Associated functions that aren't methods are often used for
     * constructors that will return a new instance of the struct.
     * These are often called 'new', but 'new' isn't a special name and
     * isn't built into the language. For example, we could choose to
     * provide an associated function named 'square' that would have
     * one dimension parameter and use that as both width and height,
     * thus making it easier to create a square 'Rectangle' rather than
     * having to specify the same value twice. */
    fn square(size: u32) -> Self {
        //                   ^^ This function returns a 'Rectangle' instance
        // v This appearance of 'Self' is a constructor for 'Rectangle'
        Self {
            width: size,
            height: size,
        }
    }
}

/* MULTIPLE 'impl' BLOCKS */
/* Each struct is allowed to have multiple 'impl' blocks. */
impl Rectangle {
    /* Methods can take ownership of, borrow immutably, or borrow mutably
     * the 'self' parameter just as they can any other parameter. */
    // We can also add a method that shares a name with a struct field.
    fn width(&self) -> bool {
        self.width > 0
    }
}
/* There is no reason to separate these methods into multiple 'impl' 
 * blocks here, but this is valid syntax. */

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    dbg!(&rect1); // If the 'area' method took ownership of 'self', instead of borrowing, this line would cause a compiler error

    if rect1.width() {
        println!("rect1 has a nonzero width: {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(4);
    dbg!(square);
}
