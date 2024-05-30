#[derive(Debug)] // enable the use of debug print on this struct - ':?'
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is {}", area(&rect1));
    println!("rect1 is {:#?}", rect1); // pretty print the struct with ':#?'

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    /* The 'dbg!' macro takes ownership of an expression, prints the
     * file and line number where the macro call occurs along with the
     * resultant value of the expression, and then returns ownership
     * of the value. */
    dbg!(&rect2);
    /* 'println!' macro uses a reference instead of taking ownership. */
    println!("{:?}", rect2);
    println!("{:?}", rect2); // No reference passed to the macro but no ownership issues

    /* When uncommented, the below statements cause a compiler error. */
    // dbg!(rect2);
    // println!("{:?}", rect2); // Borrow of moved value - compiler error

    /* Because 'dbg!' returns ownership, we can perform operations like
     * the statement below. */
    println!("{:?}", dbg!(rect2));
    /* Ownership of rect2 is taken by 'dbg!' macro but then ownership
     * is returned to the place where it was called, then being used
     * as the argument to the string literal formatting. */
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}