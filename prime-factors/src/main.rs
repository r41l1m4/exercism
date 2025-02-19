pub fn factors(n: u64) -> Vec<u64>{
    let primes_before_n = primes_until_n(n);
    let mut factors: Vec<u64> = vec![];

    for prime in primes_before_n {
        let mut tmp = n;
        loop {
            if tmp != 0 && tmp % prime != 0 {
                break;
            }
            tmp /= prime;
            factors.push(prime);
        }
        if tmp == 1 {
            break;
        }
    }

    factors
}

fn primes_until_n(n: u64) -> Vec<u64> {
    if n > 1_000_000 {
        return (2..=(n / 100_000)).filter(|num| is_prime(*num)).collect()
    }
    if n > 50_000 {
        return (2..=(n / 10)).filter(|num| is_prime(*num)).collect()
    }
    (2..=n).filter(|num| is_prime(*num)).collect()
}

fn is_prime(n: u64) -> bool {
    match n {
        1 => false,
        _ => {
            if (2..=(n / 2)).any(|num| n % num == 0) {
                return false;
            }
            true
        }
    }
}

fn main() {
    println!("{}", is_prime(2));
    println!("{}", is_prime(93_819_012_551));
    // println!("{:?}", primes_until_n(93_819_012_551));
    println!("{:?}", primes_until_n(2));
    println!("{:?}", factors(2));
    println!("{:?}", factors(60));
    println!("{:?}", factors(625));
    println!("{:?}", factors(901_255));
    println!("{:?}", factors(93_819_012_551));
}