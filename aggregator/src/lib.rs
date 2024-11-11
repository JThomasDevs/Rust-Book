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
        format!("{}, by {} ({})", self.headline, self.author,
        self.location)
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

// Continue @ Default Implementations - Section 10.2 
