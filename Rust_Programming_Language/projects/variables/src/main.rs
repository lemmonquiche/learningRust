fn main() {
    // Chapter 3.1 and 3.2
    // // This will not work because x is immutable and will throw a compiler error
    // // let x = 5;
    // // println!("The value of x is: {x}");
    // // x = 6;
    // // println!("The value of x is: {x}");

    // // Instead we explicitly say x will be mutable
    // // let mut x = 5;
    // // println!("The value of x is: {x}");
    // // x = 6;
    // // println!("The value of x is: {x}");

    // // Constant type, can never be mutable
    // // Observe how they are declared vs variables
    // // Must be a constant expression, not the result of a value that is computed at runtime
    // // Naming convention is capital with underscores between words
    // // Can be a global value, unlike let
    // // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // // Shadowing
    // // Allows us to transform values that then become immutable unless we explicitly call let as we
    // // are effecting creating a new variable. This also allows us to change the type during
    // // shadowing, as with the spaces example
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // // Does not work as the type changes from a string to usize
    // //let mut spaces = "    ";
    // //spaces = spaces.len()

    // let spaces = "    ";
    // let spaces = spaces.len();

    // println!("Number of spaces: {spaces}");

    // //Numeric operations
    // let sum = 5 + 10;
    // println!("Sum: {sum}");

    // //subtraction
    // let difference = 95.5 - 4.3;
    // println!("Difference: {difference}");

    // // multiplication
    // let product = 4 * 30;
    // println!("Product: {product}");

    // //division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // println!("Quotient: {quotient} and truncated: {truncated}");
    // 
    // // remainder
    // let remainder = 43 % 5;
    // println!("Remainder: {remainder}");

    // // booleans
    // let t = true;

    // let f: bool = false;

    // println!("T {t} and F {f}");

    // // characters
    // // Important part here is they are specified with ' instead of " for strings
    // // Since they are four bytes, they actually can fit much more than ascii and can be used for
    // // accented letters, characters, and emojis
    // let c = 'Z';
    // let z: char = 'Z';

    // println!("c = {c} and z = {z}");

    // // Compound types can group multiple values into one type
    // // Tuples are fixed length once declared
    // // Declare with (comma separated values)
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // // can use pattern matching to destructure the tuple
    // let (a, b, c) = tup;
    // println!("The value of b is {b}");

    // // accessing tuples directly using . and is 0 indexed
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;

    // // can make mutable in the sense of changing the values, not the size/length of the tuple
    // let mut e: (i32, i32) = (1, 2);
    // e.0 = 0;
    // e.1 += 5;

    // println!("Updated e tuple: {0}, {1}", e.0, e.1);

    // // Array type
    // // Each element must have the same type and cannot change size/length
    // // Useful for stack collection
    // let f = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];
    // let g: [i32; 5] = [1, 2, 3, 4, 5]; // the 5 after the colon indicates how many/size
    // let h = [3; 5]; // will result in [3, 3, 3, 3, 3] ie repeat of that value x number of times
    // 
    // // To access arrays, similar to "normal" conventions and is 0 indexed
    // let first = f[0];
    // let second = f[1];

    // // Invalid accessing will result in runtime errors
    // // let sixth = f[5];
    // // Above will not compile, but there could be cases like sixth = f[x] where x is a variable that
    // // could be changing
    // // For memory safety, Rust will stop running and not allow the memory to be accessed unlike
    // // certain other languages that will continue running

}
