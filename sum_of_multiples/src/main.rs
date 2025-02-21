use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let base_multiples_list = factors.iter()
        .map(|base_value| multiples_of_base_value(*base_value, limit))
        .collect();

    merge_multiple_list(base_multiples_list).into_iter().sum()
}

fn merge_multiple_list(multiples_list: Vec<Vec<u32>>) -> HashSet<u32> {
    multiples_list.into_iter().flatten().collect()
}

fn multiples_of_base_value(base_value: u32, level: u32) -> Vec<u32> {
    (1..level).filter(|n| {
        if base_value == 0 {
            return false;
        }
        n % base_value == 0 }).collect()
}

fn main() {
    let vec1 = multiples_of_base_value(3, 20);
    println!("{:?}", vec1);

    let vec2 = multiples_of_base_value(5, 20);
    println!("{:?}", vec2);

    let multiples = vec![vec1, vec2];
    let merged_multiples = merge_multiple_list(multiples);
    println!("{:?}", merged_multiples);

    let base_values = vec![3, 5];
    println!("{:?}", sum_of_multiples(20, &base_values));

    let factors = &[3, 0];
    let limit = 4;
    let output = sum_of_multiples(limit, factors);
    let expected = 3;
    assert_eq!(output, expected);
}
