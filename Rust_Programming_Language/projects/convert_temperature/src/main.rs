/* This program will convert temperatures between Fahrenheit and Celsius using distinct functions
 */

// Converts Fahrenheit to Celsius
fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

fn c_to_f(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

fn main() {
    let mut temp_f: f64 = 50.0;

    println!("Starting temperature is {temp_f} Fahrenheit");

    let mut temp_c: f64 = f_to_c(temp_f);

    println!("Which is {temp_c} Celius");

    if temp_f == c_to_f(temp_c) {
        println!("The functions are working as intended!")
    } else {
        println!("There is a bug somewhere!");
    }

    println!("We can also use expressions");

    temp_f = 95.0;

    temp_c = {
        (temp_f - 32.0) * 5.0/9.0
    };

    println!("Temp f is {temp_f} and in C it is {temp_c}");

    let temp_f_expression = {
        (temp_c * 9.0/5.0) + 32.0
    };

    if temp_f == temp_f_expression {
        println!("The expressions work!");
    } else {
        println!("Bug in the expressions");
    }

}
