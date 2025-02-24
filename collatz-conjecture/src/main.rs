fn collatz(n: u64) -> Option<u64> {
    if n == 0 {return None;}
    let mut n_tmp = n;
    let mut steps = 0;

    while n_tmp != 1 {
        match n_tmp % 2 == 0 {
            true => n_tmp /= 2,
            false => n_tmp = (3 * n_tmp) + 1,
        }
        steps += 1;
    }
    Some(steps)
}

fn main() {
    println!("{:?}", collatz(12));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(0));
    println!("{:?}", collatz(16));
    println!("{:?}", collatz(1_000_000));
}
