use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max { return None; }

    let combinations_from_range = combinations_from_range(vec![min, max]);
    let products_from_range = products_from_combinations(&combinations_from_range);
    let mut palindromes=  palindromes(products_from_range);
    if palindromes.is_empty() { return None; }
    palindromes.sort();

    let palindromes_min_max = [palindromes[0], palindromes[palindromes.len() - 1]];

    Some((
        Palindrome {
            value: palindromes_min_max[0],
            factors: products_of_palindrome(palindromes_min_max[0], &combinations_from_range),
        },
        Palindrome {
            value: palindromes_min_max[1],
            factors: products_of_palindrome(palindromes_min_max[1], &combinations_from_range),
        }
    ))
}

fn palindromes(products_from_range: Vec<u64>) -> Vec<u64> {
    products_from_range.into_iter()
        .filter(|n| {
            let n_rev = n.to_string().chars().rev().collect::<String>();
            n == &n_rev.parse().unwrap_or(0)
        }).collect()
}

fn products_from_combinations(combinations: &Vec<Vec<u64>>) -> Vec<u64> {
    combinations.iter()
        .map(|pair| pair[0] * pair[1])
        .unique()
        .collect::<Vec<u64>>()
}

fn combinations_from_range(range: Vec<u64>) -> Vec<Vec<u64>> {
    (range[0]..=range[1]).combinations_with_replacement(2)
        .collect()
}

fn products_of_palindrome(palindrome: u64, combinations_from_range: &Vec<Vec<u64>>) -> HashSet<(u64, u64)> {
    combinations_from_range.iter()
        .filter_map(|pair| {
            (pair.iter().product::<u64>() == palindrome).then_some((pair[0], pair[1]))
        })
        .collect()
}