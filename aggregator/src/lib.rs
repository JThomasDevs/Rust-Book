/* TRAITS: Defining Shared Behavior */
/* A 'trait' defines the functionality a particular type has and can
 * share with other types. We can use traits to define shared behavior
 * in an abstract way. We can use 'trait bounds' to specify that a
 * generic type can be any type that has certain behavior. */

/* Defining a Trait */
/* A type's behavior consists of the methods we can call on that type.
 * Different types share the same behavior if we can call the same
 * methods on all of those types. Trait definitions are a way to group
 * method signatures together to define a set of behaviors necessary to
 * accomplish some purpose.
 *
 * For example, let's say we have multiple structs that hold various
 * kinds and amounts of text: a 'NewsArticle' struct that holds a news
 * story filed in a particular location and a 'Tweet' that can have, at
 * most, 280 characters along with metadata that indicates whether it
 * was a new tweet, a retweet, or a reply to another tweet.
 *
 * We want to make a media aggregator library crate named 'aggregator'
 * that can display summaries of data that might be stored in a
 * 'NewsArticle' or 'Tweet' instance. To do this, we need a summary
 * from each type, and we'll request that summary by calling a
 * 'summarize' method on an instance. The below example shows the
 * definition of a public 'Summary' trait that expresses this behavior.
 */
pub trait Summary {
    fn summarize(&self) -> String;
}
/* Here, we declare a trait using the 'trait' keyword and then the
 * trait's name, which is 'Summary' in this case. We also delcare the
 * trait as 'pub' so that crates depending on this crate can make use
 * of this trait too, as we'll see in a few examples. Inside the curly
 * brackets, we declare the method signatures that describe the
 * behaviors of the types that implement this trait, which in this case
 * is 'fn summarize(&self) -> String;'.
 *
 * After the method signature, instead of providing an implementation
 * within curly brackets, we use a semicolon. Each type implementing
 * this trait must provide its own custom behavior for the body of the
 * method. The compiler will enforce that any type that has the
 * 'Summary' trait will have the method 'summarize' defined with this
 * signature exactly.
 *
 * A trait can have multiple methods in its body: the method signatures
 * are listed one per line, and each line ends in a semicolon. */

/* Implementing a Trait on a Type */
/* Now that we've defined the desired signatures of the 'Summary'
 * trait's methods, we can implement it on the types in our media
 * aggregator. The below example shows an implementation of the
 * 'Summary' trait on the 'NewsArticle' struct that uses the headline,
 * the author, and the location to create the return value of
 * 'summarize'. For the 'Tweet' struct, we define 'summarize' as the
 * username followed by the entire text of the tweet, assuming that the
 * tweet content is already limited to 280 characters. */
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
/* Implementing a trait on a type is similar to implementing regular
 * methods. The difference is that after 'impl', we put the trait name
 * we want to implement, then use the 'for' keyword, and then specify
 * the name of the type we want to implement the trait for. Within the
 * 'impl' block, we put the method signatures that the trait definition
 * has defined. Instead of adding a semicolon after each signature, we
 * use curly brackets and fill in the method body with the specific
 * behavior that we want the methods of the trait to have for the
 * particular type.
 *
 * Now that the library has implemented the 'Summary' trait on
 * 'NewsArticle' and 'Tweet', users of the crate can call the trait
 * methods on instances of 'NewsArticle' and 'Tweet' in the same way we
 * call regular methods. The only difference is that the user must
 * bring the trait into scope as well as the types. An example
 * of how a binary crate could use our 'aggregator' library crate can be
 * found in aggregator/src/main.rs.
 *
 * Other crates that depend on the 'aggregator' crate can also bring the
 * 'Summary' trait into scope to implement 'Summary' on their own types.
 * One restriction to note is that we can implement a trait on a type only
 * if either the trait or the type, or both, are local to our crate. For
 * example, we can implement standard library traits like 'Display' on a
 * custom type like 'Tweet' as part of our 'aggregator' crate
 * functionality because the type 'Tweet' is local to our 'aggregator'
 * crate. We can also implement 'Summary' on 'Vec<T>' in our 'aggregator'
 * crate because the trait 'Summary' is local to our 'aggregator' crate.
 *
 * But we can't implement external traits on external types. For example,
 * we can't implement the 'Display' trait on 'Vec<T>' within our
 * 'aggregator' crate because 'Display' and 'Vec<T>' are both defined in
 * the standard library and aren't local to our 'aggregator' crate. This
 * restriction is part of a property called 'coherence', and more
 * specifically the 'orphan rule', so named because the parent type is not
 * present. This rule ensures that other people's code can't break your
 * code and vice versa. Without the rule, two crates could implement the
 * same trait for the same type, and Rust wouldn't know which
 * implementation to use. */

/* Default Implementations */
/* Sometimes, it's useful to have default behavior for some or all of the
 * methods in a trait instead of requiring implementations for all methods
 * on every type. Then, as we implement the trait on a particular type, we
 * can keep or override each method's default behavior.
 *
 * In the below example, we specify a default string for the 'summarize'
 * method of the 'Summary' trait instead of only defining the method
 * signature. */
pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}
/* To use a default implementation to summarize instances of 'NewsArticle',
 * we specify an empty 'impl' block with 'impl Summary for NewsArticle {}'.
 */
impl Summary2 for NewsArticle {}
/* Even though we're no longer defining the 'summarize' method on
 * 'NewsArticle' directly, we've provided a default implementation and
 * specified that 'NewsArticle' implemenets the 'Summary' trait. As a
 * result, we can still call the 'summarize' method on an instance of
 * 'NewsArticle', like this: */
pub fn news_summary() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best 
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize2());
}
/* Creating a default implementation doesn't require us to change anything
 * about the implementation of 'Summary' on 'Tweet' above. The reason is
 * that the syntax for overriding a default implementation is the same as
 * the syntax for implementing a method that doesn't have a default
 * implementation.
 *
 * Default implementations can call other methods in the same trait, even
 * if those other methods don't have a default implementation. In this way,
 * a trait can provide a lot of useful functionality and only require
 * implementors to specify a small part of it. For example, we could define
 * the 'Summary' trait to have a 'summarize_author' method whose
 * implementation is required, and then define a 'summarize' method that
 * has a default implementation that calls the 'summarize_author' method: */
pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
/* To use this version of 'Summary', we only need to define
 * 'summarize_author' when we implement the trait on a type: */
impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
/* After we define 'summarize_author', we can call 'summarize' on instances
 * of the 'Tweet' struct, and the default implementation of 'summarize'
 * will call the definition of 'summarize_author' that we've provided.
 * Because we've implemented 'summarize_author', the 'Summary' trait has
 * given us the behavior of the 'summarize' method without requiring us to
 * write any more code. Here's what that looks like: */
pub fn tweet_summary() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize3());
}
/* Note that it isn't possible to call the default implementation from an
 * overriding implementation of that same method. */

/* Traits as Parameters */
/* Now that you know how to define and implement traits, we can explore how
 * to use traits to define functions that accept many different types.
 * We'll use the 'Summary' trait we implemented on the 'NewsArticle' and
 * 'Tweet' types above to define a 'notify' function that calls the
 * 'summarize' method on its 'item' parameter, which is of some type that
 * implements the 'Summary' trait. To do this, we use the 'impl Trait'
 * syntax, like this: */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
/* Instead of a concrete type for the 'item' parameter, we specify the
 * 'impl' keyword and the trait name. This parameter accepts any type that
 * implements the specified trait. In the body of 'notify', we can call any
 * methods on 'item' that come from the 'Sumamry' trait, such as
 * 'summarize'. We can call 'notify' and pass in any instance of
 * 'NewsArticle' or 'Tweet'. Code that call the function with any other
 * types, such as a 'String' or an 'i32', won't compile because those types
 * don't implement 'Summary'. */

/* Trait Bound Syntax */
/* The 'impl Trait' syntax works for straightforward cases but is actually
 * syntax sugar for a longer formm known as a 'trait bound'; it looks like
 * this: */
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
/* This longer form is equivalent to the example in the previous section
 * but is more verbose. We place trait bounds with the declaration of the
 * generic type parameter after a colon and inside angle brackets.
 *
 * The 'impl Trait' syntax is convenient and makes for more concise code in
 * simple cases, while the fuller trait bound syntax can express more
 * complexity in other cases. For example, we can have two parameters that
 * implement 'Summary'. Doing so with the 'impl Trait' syntax looks like
 * this: */
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("{} - {}", item1.summarize(), item2.summarize());
}
/* Using 'impl Trait' is appropriate if we want this function to allow
 * 'item1' and 'item2' to have different types (as long as both types
 * implements 'Summary'). If we want to force both parameters to have the
 * same type, however, we must use a trait bound, like this: */
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("{} - {}", item1.summarize(), item2.summarize());
}
/* The generic type 'T' specified as the type of the 'item1' and 'item2'
 * parameters constrains the function such that the concrete type of the
 * value passed as an argument for 'item1' and 'item2' must be the same. */

/* Specifying Multiple Trait Bounds with the + Syntax */

