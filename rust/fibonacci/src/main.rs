fn main() {
    let sum = sum_even_fibonacci(4_000_000);
    println!("{:?}", sum);
}

fn sum_even_fibonacci(max :i32) -> i32 {
    gen_until(max).iter().filter(|x| *x % 2 == 0).sum()
}

fn gen_until(max: i32) -> Vec<i32> {
    match max {
        1 => vec![1],
        2 => vec![1,2],
        _ => gen_until_imp(max, vec![1,2], 1, 2),
    }
}

fn gen_until_imp(max: i32, mut seq: Vec<i32>, n1: i32, n2: i32) -> Vec<i32> {
    let n3 = n1 + n2;
    if n3 < max {
        seq.push(n3);
        gen_until_imp(max, seq, n2, n3)
    } else {
        seq
    }
}