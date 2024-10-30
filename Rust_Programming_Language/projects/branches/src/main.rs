/* Chapter 3.5: Control flow
 */

fn main() {
    let number = 7;

    if number < 5 {
        // these blocks of code are called arms
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // conditions must be bool and will not try to convert non-boolean to a boolean
    // if number {}
    // will not compile, instead could do something like
    if number != 0 {
        println!("Number was something other than zero");
    }

    // Multiple conditions
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // To note, Rust has something called match that can be used for many conditions to help with
    // code readability among other things
    
    // if is an expression and can be used within statements
    let condition = true;
    let number_2 = if condition { 5 } else { 6 }; // to note, expression must result in the same
                                                  // type for this to compile

    println!("The value of number_2 is {number_2}");

    // Loops
    
    /* the following results in an infinite loop of again!
     * loop {
     *     println!{"again!"};
     * }
     */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this semi colon is optional
        }
    };

    println!("The result is {result}");

    // Loop labels to label multiple loops
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    // while loops
    let mut number_3 = 3;

    while number_3 != 0 {
        println!("{number_3}!");
        number_3 -=1 ;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1;
    }

    // While the above works, it is error prone
    // Instead do this:
    for element in a {
        println!("the value is {element}");
    }

    // liftoff example using for instead
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
