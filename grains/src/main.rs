pub fn square(s: u32) -> u64 {
    match s {
        (1..=64) => {
            let mut acc: u64 = 1;
            (2..=s).for_each(|_| acc *= 2);

            acc
        },
        _ => panic!("Only numbers between 1 and 64 (inclusive), my love."),
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 1;
    let mut acc: u64 = 1;

    (2..=64).for_each(|_| {
        acc *= 2;
        sum += acc;
    });

    sum
}

fn main() {
    println!("{}", square(10));
    println!("{}", total());
}
