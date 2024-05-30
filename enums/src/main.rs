/* Where structs give you a way of grouping together related fields and 
 * data, like a 'Rectangle' with its 'width' and 'height', enums give 
 * you a way of saying a value is one of a possible set of values. For 
 * example, we may want to say that 'Rectangle' is one of a set of 
 * possible shapes that also includes 'Circle' and 'Triangle'. To do
 * this, Rust allows us to encode these possibilities as an enum. */
 
/* As an example of a situation in which we might want to use enums
 * as opposed to structs, imagine we need to perform work with IP
 * addresses. Currently, two major standards are used for IP addresses: 
 * IPV4 and IPV6. Because these are the only possibilities for an IP 
 * address that our program will come across, we can 'enumerate' all 
 * possible variants, which is where enumeration gets its name. 
 * 
 * Any IP address can either be IPV4 or IPV6, but not both at the same
 * time. That property of IP addresses makes the enum data structure
 * appropriate because an enum value can only be one of its variants. 
 * Both IPV4 and IPV6 addresses are still fundamentally IP addresses, so
 * they should be treated as the same type when the code is handling 
 * situations that apply to any kind of IP address. 
 * 
 * We can express this concept in code by defining an 'IpAddrKind' enum 
 * and listing the possible kinds an IP address can be, 'V4', and 'V6'. 
 * These are the variants of the enum. */
#[derive(Debug)]
enum IpAddrKind {
   V4,
   V6,
}
/* 'IpAddrKind' is now a custom data type that we can use elsewhere 
 *in our code */

/* Using enums has even more advantages. Thinking more about our IP 
 * address type, at the moment we don't have a way to store the actual 
 * IP address data - we only know what kind it is.
 * A variant can be assigned a value by specifying the type(s) of values 
 * contained in the variant in parentheses after the variant name. */
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

/* There's another advantage to using an enum rather than a struct: 
 * each variant can have different types and amounts of associated 
 * data. V4 IP addresses will always have four numeric components 
 * that will have values between 0-255. If we wanted to store 'V4' 
 * addresses as four 'u8' values but still express 'V6' addresses as 
 * one 'String' value, we wouldn't be ables to with a struct. */
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

/* Now lets look at an example of an enum that has a wide variety of 
 * types embedded in its variants. */
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/* One more similarity between structs and enums is that we can define 
 * methods on enums, just as we can with structs. */
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    /* ENUM VALUES */
    /* We can create instances of each of the two variants 
     * of 'IpAddrKind' */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    /* Note that the variants of the enum are namespaced under its 
     * identifier, and we use a double colon to separate the two. This 
     * is useful because now both values are of the same type: 
     * 'IpAddrKind'. Because of this, we can define a function that 
     * takes any 'IpAddrKind'. */
    dbg!(route(four));
    dbg!(route(six));

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    /* Above, we attach data directly to each variant of the enum, so 
     * there is no need for a struct to hold the data.
     * In this example, it's also easier to see another detail of how 
     * enums work - the name of each enum variant that we define also 
     * becomes a function that constructs an instance of the enum. That 
     * is, 'IpAddr::V4()' is a function call that takes a 'String' 
     * argument and returns an instance of the 'IpAddr' type. We 
     * automatically get this constructor function defined as a result 
     * of defining the enum. */
    
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    /* Calling a function on an enum */
    let msg = Message::Write(String::from("hello"));
    msg.call();

    /* The 'Option' enum */
    /* The 'Option' type encodes the very common scenario in which a 
     * value could be either something or it could be nothing.
     * 
     * For example, if you request the first item in a non-empty list, 
     * you would get a value. If you request the first item in an empty 
     * list, you would get nothing. Expressing this concept in terms of 
     * the type system means the compiler can check whether you've 
     * handled all the cases you should be handling - this functionality 
     * can prevent bugs that are extremely common in other programming 
     * languages. 
     * 
     * Programming language design is often thought of in terms of which 
     * features you include, but the features you exclude are important 
     * too. Rust doesn't have have the null feature that many other 
     * languages have. 'Null' is a value that means there is no value 
     * there. In languages with null, variables can always be in one of
     * two states: null or not-null.
     * 
     * The problem with null values is that if you try to use a null 
     * value as a not-null value, you'll get an error of some kind. 
     * Because this null or not-null property is pervasive, it's 
     * extremely easy to make this kind of error. However, the concept 
     * that null is trying to express is still a useful one: a null is 
     * a value that is currently invalid or absent for some reason.
     * The problems lies not with the concept but rather with the 
     * particular implementation. As such, Rust does not have nulls, but 
     * it does have an enum that can encode the concept of a value being 
     * present or absent. This enum is 'Option<T>', and its definition 
     * looks like the following: */
    enum _Option<T> {
        None,
        Some(T),
    }
    /* The 'Option<T>' enum is so useful that it's even included in the 
     * prelude. Its variants are also included in the prelude: you can 
     * use 'Some' and 'None' directly without the 'Option::' prefix. The 
     * 'Option<T>' enum is still just a regular enum, and 'Some(T)' and 
     * 'None' are still variants of type 'Option<T>'.
     * 
     * The '<T>' syntax is a 'generic type parameter'. While generics 
     * are covered later, just know that it means that the 'Some' 
     * variant of the 'Option' enum can hold one piece of data of any 
     * type, and that each concrete type that gets used in place of 'T' 
     * makes the overall 'Option<T>' type a different type. */
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
    /* The type of 'some_number' is 'Option<i32>'. The type of 
     * 'some_char' is 'Option<char>', which is a different type. 
     * Rust can infer these types because we've specified a value inside 
     * the 'Some' variant. For 'absent_number', Rust requires us to 
     * annotate the overall 'Option' type: the compiler can't infer the 
     * type that the corresponding 'Some' variant will hold by looking 
     * at a 'None' value. Above, we tell the compiler that we mean for 
     * 'absent_number' to be of type 'Option<i32>'.
     * 
     * When we have a 'Some' value, we know that a value is present and 
     * the value is held within the 'Some'. When we have a 'None' value, 
     * in some sense it means the same thing as null: we don't have a 
     * valid value. So why use 'Option<T>' instead of 'null'?
     * 
     * In short, because 'Option<T>' and 'T'(where 'T' can be any type) 
     * are different types, the compiler won't let us use an 'Option<T>' 
     * value as if it were definitely a valid value. */
    // The following statements, when uncommented, do not compile
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    /* If we wish to perform 'T' operations with the value contained in 
     * the 'Some' variant, we need to convert the 'Option<T>' to a 'T'.
     * Generally, this helps catch one of the most common issues with 
     * null: assuming that something isn't null when it actually is. 
     * 
     * To convert an Option<T> to a T, we can use the .unwrap() 
     * function. The 'Option' type has many useful associated functions 
     * defined in its documentation.
     * Access these docs by running:
     * rustup doc --std
     * and navigating to:
     * std::option::Option 
     * 
     * However, in general, in order to use an 'Option<T>' value, we 
     * want to have code that will handle each variant. This can be done 
     * using a 'match' expression to execute a block of code depending 
     * on the Option variant that is matched. */
    let opt: Option<u8> = None;
    match opt {
        Some(i) => { println!("opt is Some! Value of opt: {i}") },
        None => { println!("opt in None!") },
    };
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
