// Prints out the twelve days of christmas song

fn main() {
    let number_days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    let true_love = "my true love gave to me";

    for day in 0..11 {
        println!("One the {} day of christmas", number_days[day]);
        println!("{true_love}");
        for gift in (0..day+1).rev() {
            if day > 0 && gift == 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
        println!("");
    }
}
