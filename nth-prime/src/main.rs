pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut result = 0u32;

    for num in 2.. {
        if primes.len() == (n + 1) as usize {
            result = primes[(n) as usize];
            break;
        }

        if is_prime(num) {
            primes.push(num);
        }
    }

    result
}

fn is_prime(n: u32) -> bool {
    if (2..=(n / 2)).any(|x| n % x == 0) {
        return false;
    }

    true
}

fn main() {
    let output = nth(0);
    let expected = 2;
    assert_eq!(output, expected);

    let output = nth(1);
    let expected = 3;
    assert_eq!(output, expected);

    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);

    let output = nth(10_000);
    let expected = 104_743;
    assert_eq!(output, expected);
}