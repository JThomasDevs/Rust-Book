fn main() {
    /* Sometimes, bad things happen in your code, and there's nothing you can
     * do about it. In these cases, Rust has the panic! macro. There are two
     * ways to cause a panic in practice: by taking an action that causes our
     * code to panic (such as accessing an array past the end) or by
     * explicitly calling the panic! macro. In both cases, we cause a panic
     * in our program. By default, these panics will print a failure message,
     * unwind, clean up the stack, and quit. Via an environment variable, you
     * can also have Rust display the call stack when a panic occurs to make
     * it easier to track down the source of the panic. */
    // When uncommented, the following line causes a panic
    // panic!("crash and burn");

    /* The call to 'panic!' causes the error message contained in the last
     * two lines of the resulting backtrace. The first line shows the
     * panic message and the place in the source code where the panic
     * occurred.
     *
     * In this case, the line indicated is part of our code, and if we go to
     * that line, we see the 'panic!' macro call. In other cases, the
     * 'panic!' call might be in code that our code calls, and the filename
     * and line number reported by the error message will be someone else's
     * code where the 'panic!' macro is called, not the line of our code that
     * eventually led to the 'panic!' call. We can use the backtrace of the
     * functions the 'panic!' call came from to figure out the part of our
     * code that is causing the problem.
     *
     * Let's look at another example to see what it's like when a 'panic!'
     * call comes from a library because of a bug in our code instead of from
     * our code calling the macro directly. The following example has some
     * code that atttempts to access and index in a vector beyond the range
     * of valid indexes. */
    let _v = vec![1, 2, 3];
    // When uncommented, the following line causes a panic
    //_v[99];
    /* Above, we attempt to access the 100th element of our vector, but the
     * vector has only 3 elements. In this situation, Rust will panic. Using
     * '[]' is supposed to return an element, but if you pass an invalid
     * index, there's no element that Rust could return here that would be
     * correct.
     *
     * In C, attempting to read beyond the end of a data structure is
     * undefined behavior. You might get whatever is at the location in
     * memory that would correspond to that element in the data structure,
     * even though the memory doesn't belong to that structure. This is
     * called 'buffer overread' and can lead to security vulnerabilities if
     * an attacker is able to manipulate the index in such a way as to read
     * data they shouldn't be allowed to that is stored after the data
     * structure.
     *
     * To protexct your program from this sort of vulnerability, if you try
     * to read an element at an index that doesn't exist, Rust will stop
     * executing and refuse to continue. */

    /* Recovering from Errors with Result */
    /* Most errors aren't serious enough to require the program to stop
     * entirely. Sometimes, when a function fails, it's for a reason that you
     * can easily interpret and respond to. For example, if you try to open a
     * file and that operation fails because the file doesn't exist, you
     * might want to create the file instead of terminating the process.
     * This can be done with the 'Result' enum. The 'Result' enum is defined
     * with two variants: 'Ok' and 'Err'.
     *
     * Let's call a function that returns a 'Result' value because the
     * function could fail. */
    use std::fs::File;

    let _greeting_file_result = File::open("hello.txt");
    /* The return type of 'File::open' is a 'Result<T,E>'. The generic
     * parameter 'T' has been filled in by the implementation of 'File::open'
     * with the type of the success value, 'std::fs'File', which is a file
     * handle. The type of 'E' used in the error value is 'std::io::Error'.
     * This return type means the call to 'File::open' might succeed and
     * return a file handle that we can read from or write to. The function
     * call also might fail: for example, the file might not exist, or we
     * might not have permission to access the file. The 'File::open'
     * function needs to have a way to tell us whether it succeeded or failed
     * and at the same time give us either the file handle or error
     * information. This information is exactly what the 'Result' enum
     * conveys.
     *
     * In the case where 'File::open' succeeds, the value in the variable
     * 'greeting_file_result' will be an instance of 'Ok' that contains a
     * file handle. In the case where it fails, the value in
     * 'greeting_file_result' will be an instance of 'Err' that contains more
     * information about the kind of error that happened.
     *
     * We need to add to the code in the above example to take different
     * actions depending on the value 'File::open' returns. The following
     * example shows one way to handle the 'Result' using a basic tool, the
     * 'match' expression. */
    //    let _greeting_file = match _greeting_file_result {
    //        Ok(file) => file,
    //        Err(error) => {
    //            panic!("Problem opening the file: {:?}", error)
    //        }
    //    };
    /* Note that, like the 'Option' enum, the 'Result' enum and its variants
     * have been brought into scope by the prelude, so we don't need to
     * specify 'Result::' before the 'Ok' and 'Err' variants in the 'match'
     * arms.
     *
     * When the result is 'Ok', this code will return the inner 'file' value
     * out of the 'Ok' variant, and we then assign that file handle value to
     * the variable 'greeting_file'. After the 'match', we can use the file
     * handle for reading or writing.
     *
     * The other arm of the 'match' handles the case where we get an 'Err'
     * value from 'File::open'. In this example, we've chosen to call the
     * 'panic!' macro. If there's no file named 'hello.txt' in our current
     * directory and we run this code, we'll see the backtrace from the
     * 'panic!' macro. */

    /* Matching on Different Errors */
    /* The code in the above example will 'panic!' no matter why
     * 'File::open' failed. However, we want to take different actions for
     * different failure reasons: if 'File::open' failed because the file
     * doesn't exist, we want to create the file and return the handle to
     * the new file. If 'File::open' failed for any other reason - for
     * example, because we don't have permission to open the file - we
     * still want the code to 'panic!' in the same way as it did in the
     * above example. For this, we can use an inner 'match' expression. */
    //    use std::io::ErrorKind;
    //
    //    let greeting_file_result = File::open("hello.txt");
    //
    //    let _greeting_file = match greeting_file_result {
    //        Ok(file) => file,
    //        Err(error) => match error.kind() {
    //            ErrorKind::NotFound => match File::create("hello.txt") {
    //                Ok(fc) => fc,
    //                Err(e) => panic!("Problem creating the file: {:?}", e),
    //            },
    //            other_error => {
    //                panic!("Problem opening the file: {:?}", other_error);
    //            }
    //        },
    //    };
    /* The type of the value that 'File::open' returns inside the 'Err' is
     * 'io::Error', which is a struct provided by the standard library.
     * This struct has a method 'kind' that we can call to get an
     * 'io::ErrorKind' value. The 'io::ErrorKind' enum is provided by the
     * standard library and has variants representing the different kinds
     * of errors that might result from an 'io' operation. The variant we
     * want to use is 'ErrorKind::NotFound', which indicates the file
     * we're trying to open doesn't exist yet. So we match on
     * '_greeting_file_result', but we also have an inner match on
     * 'error.kind()'.
     *
     * The condition we want to check in the inner match is whether the
     * value returned by 'error.kind()' is the 'NotFound' variant of the
     * 'ErrorKind' enum. If it is, we try to create the file with
     * 'File::create'. However, because 'File::create' could also fail, we
     * need a second arm in the inner 'match' expression. When the file
     * can't be created, a different error message is printed. The second
     * arm of the outer 'match' stays the same, so the program panics on
     * any error besides the missing file error. */

    /* Shortcuts for Panic on Error: unwrap and expect */
    /* Using 'match' works well enough, but it can be a bit verbose and
     * doesn't always communicate intent well. The 'Result<T,E>' type has many
     * helper methods defined on it to do various, more specific tasks. The
     * 'unwrap' method is a shortcut method implemented just like the 'match'
     * expression we used above. If the 'Result' value is the 'Ok' variant,
     * 'unwrap' will return the value inside the 'Ok'. If the 'Result' is the
     * 'Err' variant, 'unwrap' will call the 'panic!' macro for us. */
    // let _greeting_file = File::open("hello.txt").unwrap();

    /* Similarly, the 'expect' method lets us also choose the 'panic!' error
     * message. Using 'expect' instad of 'unwrap' and providing good error
     * messages can convey your intent and make tracking down the source of a
     * panic easier. */
    // let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    /* We use 'expect' in the same way as 'unwrap': to return the file handle
     * or call the 'panic!' macro. The error message used by 'expect' in its
     * call to 'panic!' will be the parameter that we pass to 'expect', rather
     * than the default 'panic!' message that 'unwrap' uses.
     *
     * In production-quality code, most Rustaceans choose 'expect' rather than
     * 'unwrap' and give more context about why the operation is expected to
     * always succeed. That way, if your assumptions are ever proven wrong,
     * you have more information to use in debugging. */

    /* Propagating Errors */
    /* When a function's implementation calls something that might fall,
     * instead of handling the error within the function itself, you can
     * return the error to the calling code so that is can decide what to do.
     * This is known as 'propagating' the error and gives more control to the
     * calling code, where there might be more information or logic that
     * dictates how the error should be handled than what you have available
     * in the context of your code.
     *
     * The below example shows the process of propagating errors manually. */
    use std::io::{self, Read};

    fn _read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    /* This function can be written in a much shorter way, but we're going to
     * start by doing a lot of it manually in order to explore error handling;
     * at the end we'll use the shorter way. Let's look at the return type of
     * the function first: 'Result<String, io::Error>'. This means the
     * function is returning a value of the type 'Result<T, E>' where the
     * generic parameter 'T' has been filled in with the concrete type
     * 'String', and the generic type 'E' has been filled in with the concrete
     * type 'io::Error'.
     *
     * If this function succeeds without any problems, the code that calls
     * this function will receive an 'Ok' value that holds a 'String' — the
     * username that this function read from the file. If this function
     * encounters any problems, the calling code will receive an 'Err' value
     * that holds an instance of 'io::Error' that contains more information
     * about what the problems were. We chose 'io::Error' as the return type
     * of this function because that happens to be the type of the error value
     * returned from both of the operations we’re calling in this function’s
     * body that might fail: the 'File::open' function and the 'read_to_string'
     * method.
     *
     * The body of the function starts by calling the 'File::open' function.
     * Then we handle the 'Result' value with a 'match' expression. If
     * 'File::open' succeeds, the file handle in the pattern variable 'file'
     * becomes the value in the mutable variable 'username_file' and the
     * function continues. In the 'Err' case, instead of calling 'panic!', we
     * use the 'return' keyword to return early out of the function entirely
     * and pass the error value from 'File::open', now in the pattern variable
     * 'e', back to the calling code as this function’s error value.
     *
     * So if we have a file handle in 'username_file', the function then
     * creates a new 'String' in variable 'username' and calls the
     * 'read_to_string' method on the file handle in 'username_file' to read
     * the contents of the file into 'username'. The 'read_to_string' method
     * also returns a 'Result' because it might fail, even though 'File::open'
     * succeeded. So we need another 'match' to handle that 'Result': if
     * 'read_to_string' succeeds, then our function has succeeded, and we
     * return the username from the file that’s now in 'username' wrapped in
     * an 'Ok'. If 'read_to_string' fails, we return the error value in the
     * same way that we returned the error value in the 'match' that handled
     * the return value of 'File::open'. However, we don’t need to explicitly
     * say 'return', because this is the last expression in the function.
     *
     * The code that calls this code will then handle getting either an 'Ok'
     * value that contains a username or an 'Err' value that contains an
     * 'io::Error'. It’s up to the calling code to decide what to do with those
     * values. If the calling code gets an 'Err' value, it could call 'panic!'
     * and crash the program, use a default username, or look up the username
     * from somewhere other than a file, for example. We don’t have enough
     * information on what the calling code is actually trying to do, so we
     * propagate all the success or error information upward for it to handle
     * appropriately.
     *
     * This pattern of propagating errors is so common in Rust that Rust
     * provides the question mark operator '?' to make this easier. */

    /* A Shortcut for Propagating Errors: the ? Operator */
    /* The below example shows an implementation of '_read_username_from_file'
     * that has the same functionality as the above example, but this
     * implementation uses the '?' operator. */
    fn _read_username_from_file1() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    /* The '?' placed after a 'Result' value is defined to work in almost the
     * same way as the 'match' expressions we defined to handle the 'Result'
     * in the first implementation of this function. If the value of the
     * 'Result' is an 'Ok', the value inside the 'Ok' will get returned from
     * this expression, and the program will continue. If the value is an
     * 'Err', the 'Err' will be returned from the whole function as if we had
     * used the 'return' keyword so the error value gets propagated to the
     * calling code.
     *
     * There is a difference between what the 'match' expression from above
     * does and what the '?' operator does: error values that have the '?'
     * operator called on them go through the 'from' function, defined in the
     * 'From' trait in the standard library, which is used to convert values
     * from one type into another. When the '?' operator calls the 'from'
     * function, the error type received is converted into the error type
     * defined in the return type of the current function. This is useful
     * when a function returns one error type to represent all the ways a
     * function might fail, even if parts might fail for many different
     * reasons.
     *
     * For example, we could change the '_read_username_from_file1' function
     * in the above example to return a custom error type named 'OurError'
     * that we define. If we also define 'impl From<io::Error>' for
     * 'OurError' to construct an instance of 'OurError' from an 'io::Error',
     * then the '?' operator calls in the body of '_read_username_from_file1'
     * will call 'from' and convert the error types without needing to add
     * any more code to the function.
     *
     * In the context of the above example, the '?' at the end of the
     * 'File::open' call will return the value inside an 'Ok' to the variable
     * 'username_file'. If an error occurs, the '?' operator will return
     * early out of the whole function and give any 'Err' value to the
     * calling code. The same thing applies to the '?' at the end of the
     * 'read_to_string' call.
     *
     * The '?' operator eliminates a lot of boilerplate and makes this
     * function's implementation simpler. We could even shorten this code
     * further by chaining method calls immediately after the '?'. */
    fn _read_username_from_file2() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
    /* We've moved the creation of the new 'String' in 'username' to the
     * beginning of the function; that part hasn't changed. Instead of
     * creating a variable 'username_file', we've chained the call to
     * 'read_to_string' directly onto the result of 'File::open("hello.txt")?'.
     * We still have a '?' at the end of the 'read_to_string' call, and we
     * still return an 'Ok' value containing 'username' when both 'File::open'
     * and 'read_to_string' succeed rather than returning errors. The
     * functionality is again the same as in the above two examples; this is
     * just a different, more ergonomic way to write it.
     *
     * The below example shows a way to make this even shorter using
     * 'fs::read_to_string'. */
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
    /* Reading a file into a string is a fairly common operation, so the
     * standard library provides the convenient 'fs::read_to_string' function
     * that opens the file, creates a new 'String', reads the contentsof the
     * file, puts the contents into that 'String', and returns it. */

    /* Where the '?' Operator Can Be Used */
    /* The '?' operator can only be used in functions whose return type is
     * compatible with the value the '?' is used on. This is because the '?'
     * operator is defined to perform an early return of a value out of the
     * function, in the same manner as the 'match' expression we defined in an
     * above example. In that example, the 'match' was using a 'Result' value,
     * and the early return arm returned an 'Err(e)' value. The return type of
     * the function has to be a 'Result' so that it's compatible with this
     * 'return'.
     *
     * In the below example, let's look at the error we'll get if we use the
     * '?' operator in a 'main' function with a return type incompatible with
     * the type of the value we use '?' on: */
    // The below code, when uncommented, will not compile
    // let greeting_file = File::open("hello.txt")?;
    /* This code opens a file, which might fail. The '?' operator follows the
     * 'Result' value returned by 'File::open', but this 'main' function has a
     * return type of '()', not 'Result'.
     *
     * The compiler error given by the above code points out that we're only
     * allowed to use the '?' operator in a function that returns 'Result',
     * 'Option', or another type that implements 'FromResidual'.
     *
     * To fix the error, you have two choices. One choice is to change the
     * return type of your function to be compatible with the value you’re
     * using the '?' operator on as long as you have no restrictions preventing
     * that. The other technique is to use a 'match' or one of the
     * 'Result<T,E>' methods to handle the 'Result<T,E>' in whatever way is
     * appropriate.
     *
     * The error message also mentions that '?' can be used with 'Option<T>'
     * values as well. As with using '?' on 'Result', you can only use '?' on
     * 'Option' in a function that returns an 'Option'. The behavior of the '?'
     * operator when called on an 'Option<T>' is similar to its behavior when
     * called on a 'Result<T,E>': if the value is 'None', the 'None' will be
     * returned early from the function at that point. If the value is 'Some',
     * the value inside the 'Some' is the resulting value of the expression
     * and the function continues. */
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    println!("{:?}", last_char_of_first_line("hello again!"));
    /* This function returns 'Option<char>' because it’s possible that there
     * is a character there, but it’s also possible that there isn’t. This code
     * takes the 'text' string slice argument and calls the 'lines' method on
     * it, which returns an iterator over the lines in the string. Because this
     * function wants to examine the first line, it calls 'next' on the
     * iterator to get the first value from the iterator. If 'text' is an empty
     * string, this call to 'next' will return 'None', in which case we use '?'
     * to stop and return 'None' from '_last_char_of_first_line'. If 'text' is
     * not an empty string, 'next' will return a 'Some' value containing a
     * string slice of the first line in 'text'.
     *
     * The '?' extracts the string slice, and we can call 'chars' on that
     * string slice to get an iterator of its characters. We’re interested in
     * the last character in this first line, so we call 'last' to return the
     * last item in the iterator. This is an 'Option' because it’s possible
     * that the first line is an empty string, for example if 'text' starts
     * with a blank line but has characters on other lines, as in "\nhi".
     * However, if there is a last character on the first line, it will be
     * returned in the 'Some' variant. The '?' operator in the middle gives us
     * a concise way to express this logic, allowing us to implement the
     * function in one line. If we couldn’t use the '?' operator on 'Option',
     * we’d have to implement this logic using more method calls or a 'match'
     * expression.
     *
     * Note that you can use the '?' operator on a 'Result' in a function that
     * returns 'Result', and you can use the '?' operator on an 'Option' in a
     * function that returns 'Option', but you can’t mix and match. The '?'
     * operator won’t automatically convert a 'Result' to an 'Option' or vice
     * versa; in those cases, you can use methods like the 'ok' method on
     * 'Result' or the 'ok_or' method on 'Option' to do the conversion
     * explicitly.
     * So far, all the 'main' functions we’ve used return '()'. The 'main'
     * function is special because it’s the entry and exit point of executable
     * programs, and there are restrictions on what its return type can be for
     * the programs to behave as expected.
     *
     * Luckily, 'main' can also return a 'Result<(),E>'. The example below
     * shows what a 'main' function would look like when changed to return a
     * 'Result<(), Box<dyn Error>>' and added a return value 'Ok(())' to the
     * end. */
    use std::error::Error;

    fn _main() -> Result<(), Box<dyn Error>> {
        let _greeting_file = File::open("hello.txt")?;

        Ok(())
    }
    /* The 'Box<dyn Error>' type is a 'trait object', which is covered later.
     * For now, you can read 'Box<dyn Error>' to mean “any kind of error.”
     * Using '?' on a 'Result' value in a 'main' function with the error type
     * 'Box<dyn Error>' is allowed, because it allows any 'Err' value to be
     * returned early. Even though the body of this 'main' function will only
     * ever return errors of type 'std::io::Error', by specifying
     * 'Box<dyn Error>', this signature will continue to be correct even if
     * more code that returns other errors is added to the body of main.
     *
     * When a 'main' function returns a 'Result<(),E>', the executable will
     * exit with a value of '0' if main returns 'Ok(())' and will exit with a
     * nonzero value if 'main' returns an 'Err' value. Executables written in
     * C return integers when they exit: programs that exit successfully return
     * the integer '0', and programs that error return some integer other than
     * '0'. Rust also returns integers from executables to be compatible with
     * this convention.
     *
     * The 'main' function may return any types that implement the
     * 'std::process::Termination' trait, which contains a function 'report'
     * that returns an 'ExitCode'. Consult the standard library documentation
     * for more information on implementing the 'Termination' trait for your
     * own types. */
}
