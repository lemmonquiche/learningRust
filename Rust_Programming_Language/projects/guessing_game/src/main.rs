use std::io; // standard library that has input/output functions
use rand::Rng; // for use of the random number generator crate
use std::cmp::Ordering; // standard library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive of upper and lower
                                                               // bounds
                                                               // thread_rng a rng that is local to
                                                               // the current thread and seeded by
                                                               // the OS
    loop { //creates an infinity loop
        println!("Please input your guess between 1 and 100: ");

        let mut guess = String::new(); // mut allows variable guess to be mutable
                                       // ::new says return a new empty object of type String
                                       //   ie this is a function associated with a type

        io::stdin() // another way is std::io::stdin if use std::io was not called before
            .read_line(&mut guess) // & is referencing guess and &mut allows the reference to be
                                   // mutable as references are immutable by default
            .expect("Failed to read line :("); // since ; was not called before, this is still one line
                                               // of code instead of using io::stdin().read_line(&mut).expect()
                                               // Reminds me a bit like python in this sense
                                               // expect deals with the result of read_line as
                                               // read_line will either return Ok or Err and gives a
                                               // message when read_line results in Err
                                               // You will actually get a warning when compiling if
                                               // expect is not called telling you that the program may
                                               // not have handled an error
                                               // _ is a catchall

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Example of shadowing variables
          // guess. is the original string variable
          // trim() removes white/new space
          // parse() converts string to another type as expressed by : u32
          // originally used except() to handle non-number values, but match is a better approach
        

        println!("You guessed: {}", guess); // {} is a placeholder for a variable

        match guess.cmp(&secret_number) { // cmp compares two values and returns an Ordering enum that
                                          // we can use match to print results of the comparison to the
                                          // user
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //breaks the forloop
            }
        }
    }

    println!("The secret number is: {secret_number}");
}
