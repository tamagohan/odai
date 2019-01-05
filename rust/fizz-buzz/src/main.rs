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

#[test]
fn test_fizz_buzz() {
    assert_eq!(fizz_buzz(1), "1");
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(15), "FizzBuzz");
}
