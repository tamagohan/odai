fn main() {
    puts_fizz_buzz(100);
}

fn puts_fizz_buzz(max: i32) -> () {
    (1..max)
        .map(|num| fizz_buzz(num))
        .for_each(|word| println!("{}", word));
}

fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}
