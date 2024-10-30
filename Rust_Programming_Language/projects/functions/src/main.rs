// Chapter 3.3: Functions, 3.4: Comments
// Rust uses snake case, which means all functions and variables should be lowercase and use _ to
// separate words
// Expressions vs statements is important to understand as Rust is an expression-based language
// Function bodies are made up of a series of statements that optionally end in an expression
//      expressions evaluate to a resultant value, majority of your code generally
//          Some examples: x + y
//      statements are instructions that perform some action and do not return a value
//          Some examples: let, function definitions
/* Or do multiline comments
 * like
 * this
 */

fn main() {
    another_function(5, 'h');

    let y = 6; // this is a statement
    // let x = (let y = 6); // Will not compile as let y = 6 does not return a value that can be
    // assigned to x
   
    // This is an expression within the {} block
    let x = {
        let z = 3;
        z + 1 // expressions do not including ending semicolons!!!!!!! if you do add one, it will
              // not return the value to be bound to x as you turn it into a statement
    };

    println!("The value of x is: {x}");

    let a = five();

    println!("The value of a is {a}");

    let b = plus_one(5);

    println!("The value of b is {b}");
}

// It does not matter if this is declared before or after main, just that it is in scope to be
// called
// In function signatures, you must declare the type of each parameter
fn another_function(x: i32, unit_label: char) { // How functions are called with a parameter/arguments
    println!("The value of x is {x} and the unit is {unit_label}");
}

// The return value is synonymous with the value of the final expression in the block of the body
// of a function
// Can return early by using the return keyword and a value
fn five() -> i32 { // must return declare type if you want to return a value from a function
                   // use -> to declare the return type
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
