/* This code will generate the nth fibonacci sequence */

fn fibonacci(nth: u32) -> u32 {
    let mut fib_seq = [0, 1];
    let mut temp;

    for _i in 2..nth {
        temp = fib_seq[0] + fib_seq[1];
        fib_seq[0] = fib_seq[1];
        fib_seq[1] = temp;
    }

    fib_seq[0] + fib_seq[1]
}

fn main() {
    let mut truth = 5;
    let mut fib_result = fibonacci(5);
    println!("The {}th value is {}", 5, fib_result);

    if fib_result == truth {
        println!("fibonacci loop code works");
    } else {
        println!("there is a bug");
    }

    truth = 55; // according to google
    fib_result = fibonacci(10);
    println!("The {}th value is {}", 10, fib_result);

    if fib_result == truth {
        println!("fibonacci loop code works");
    } else {
        println!("there is a bug");
    }
}
