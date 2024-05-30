/* The type 'HashMap<K,V>' stores a mapping of keys of type 'K' to values
 * of type 'V' using a 'hashing function', which determines how it places
 * these keys and values into memory.
 *
 * Hash maps are useful when you want to look up data not by using an
 * index, as you can with vectors, but by using a key that can be of any
 * type. For example, in a game, you could keep track of each team's
 * score in a hash map in which each key is a team's name and the values
 * are each team's score. Given a team naem, you can retrieve its
 * score. */
use std::collections::HashMap;
fn main() {
    // Creating a New Hash Map
    /* One way to create an emtpy has map is using 'new' and adding
     * elements with 'insert'. */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    /* Note that we need to first 'use' the 'HashMap' from the
     * collections portion of the std library. Of the three common
     * collections, this one is the least often used, so it's not
     * included in the features brought into scope automatically in the
     * prelude.
     *
     * Just like vectors, hash maps store their data on the heap. This
     * 'HashMap' has keys of type 'String' and values of type 'i32'.
     * Like vectors, hash maps are homogenous: all of the keys must have
     * the same type as each other, and all of the values must have the
     * same type. */

    // Accessing Values in a Hash Map
    /* We can get a value out of the hash map by providing its key to
     * the 'get' method, as shown below. */
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    /* Here, 'score' will have the value that's associated with the Blue
     * team, and the result will be '10'. The 'get' method returns an
     * 'Option<&V>'; if there's no value for that key in the hash map,
     * 'get' will return 'None'. This program handles the 'Option' by
     * calling 'copied' to get an 'Option<i32>' rather than an
     * 'Option<&i32>', then 'unwrap_or' to set 'score' to zero if
     * 'scores' doesn't have an entry for the key.
     *
     * We can iterate over each key/value pair in a hash map in a similar
     * manner as we do with vectors, using a 'for' loop. */
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    /* For types that implement the 'Copy' trait, like 'i32', the values
     * are copied into the hash map. For owned values like 'String', the
     * values will be moved and the hash map will be the owner of those
     * values. */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // Attemping to use either will give a compiler error
    // println!("{field_name}");
    /* We aren't able to use 'field_name' and 'field_value' after they've
     * been moved into the hash map with the call to 'insert'.
     *
     * If we insert references to values into the hash map, the values
     * won't be moved into the hash map. The values that the references
     * point to must be valid for at least as long as the hash map is
     * valid. */

    // Updating a Hash Map
    /* Althoguh the number of key and value pairs is growable, each
     * unique key can only have one value associated with it at a time.
     *
     * When you want to change the data in a hash map, you have to decide
     * how to handle the case when a key already has a value assigned.
     * You could replace the old value with the new value, completely
     * disregarding the old value. You could keep the old value and
     * ignore the new value, only adding the new value if the key
     * doesn't already have a value. Or you could combine the old value
     * and the new value. */
    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);
    /* This code wil print '{"Blue": 25}' as the original value of '10'
     * has been overwritten */

    // Adding a Key and Value Only If a Key Isn't Present
    /* It's common to check whether a particular key already exists in
     * the hash map with a value then take the following actions:
     * if the key does exist in the hash map, the existing value should
     * remain the way it is. If the key doesn't exist, insert it and a
     * value for it.
     *
     * Hash maps have a special API for this called 'entry' that takes
     * the key you want to check as a parameter. The return value of the
     * 'entry' method is an enum called 'Entry' that represents a value
     * that might or might not exist. Let's say we want to check whether
     * the key for the Yellow team has a value associated with it. If it
     * doesn't, we want to insert the value 50, and the same for the
     * Blue team. */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:#?}", scores);
    /* The 'or_insert' method on 'Entry' is defined to return a mutable
     * reference to the value for the corresponding 'Entry' key if that
     * key exists, and if not, inserts the parameter as the new value for
     * this key and returns a mutable reference to the new value. This
     * technique is much cleaner than writing the logic ourselves and, in
     * addition, plays more nicely with the borrow checker. */

    // Updating a Value Based on the Old Value
    /* Another common use case for hash maps is to look up a key's value
     * and then update it based on the old value. For instance, the
     * example below counts how many times each word appears in some
     * text. We use a hash map with the words as keys and increment the
     * value to keep track of how many times we've seen that word. If
     * it's the first time we've seen a word, we'll first insert the
     * value '0'. */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
    /* The 'split_whitespace' method returns an iterator over sub-slices
     * separated by whitespace, of the value in 'text'. The 'or_insert'
     * method returns a mutable reference ('&mut V') to the value for
     * the specified key. Here we store that mutable reference in the
     * 'count' variable, so in order to assign to that value, we must
     * first dereference 'count' using the asterisk ('*'). The mutable
     * reference goes out of scope at the end of the 'for' loop, so all
     * of these changes are safe and allowed by the borrowing rules. */
}
