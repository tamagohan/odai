use std::borrow::Cow;

fn main() {
    puts_fizz_buzz(100);
}

fn puts_fizz_buzz(max: i32) -> () {
    (1..max)
        .map(|num| fizz_buzz(num))
        .for_each(|word| println!("{}", word));
}

fn fizz_buzz(num: i32) -> Cow<'static, str> {
    if num % 15 == 0 {
        "FizzBuzz".into()
    } else if num % 3 == 0 {
        "Fizz".into()
    } else if num % 5 == 0 {
        "Buzz".into()
    } else {
        num.to_string().into()
    }
}
