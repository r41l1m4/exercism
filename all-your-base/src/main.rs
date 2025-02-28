use crate::Error::{InvalidDigit, InvalidInputBase, InvalidOutputBase};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0, _) => return Err(InvalidInputBase),
        (1, _) => return Err(InvalidInputBase),
        (_, 1) => return Err(InvalidOutputBase),
        (_, 0) => return Err(InvalidOutputBase),
        _ => (),
    }
    from_decimal(&to_decimal(number, from_base)?, to_base)
}

fn to_decimal(number: &[u32], from_base: u32) -> Result<Vec<u32>, Error> {
    for n in number {
        if n >= &from_base { return Err(InvalidDigit(*n)); }
    }

    let result: u32 = number.iter()
        .zip((0..number.len()).rev())
        .map(|(num, pow)| num * from_base.pow(pow as u32))
        .sum();

    Ok(result.to_string()
        .chars()
        .map(|ch| ch.to_string().parse().unwrap())
        .collect::<Vec<u32>>())
}

fn from_decimal(number: &[u32], to_base: u32) -> Result<Vec<u32>, Error> {
    let mut num_tmp: u32 = String::from_iter(number.iter().map(|n| format!("{n}"))).parse().unwrap();
    let mut result: Vec<u32> = vec![];

    if num_tmp < to_base { return Ok(vec![num_tmp]); }
    loop {
        if num_tmp == 0 { break; }
        result.push(num_tmp % to_base);
        if num_tmp == 1 { break; }
        num_tmp /= to_base;
    }
    Ok(result.into_iter().rev().collect())
}

fn main() {
    println!("{:?}", from_decimal(&[4, 2], 2));
    println!("{:?}", from_decimal(&[1, 2], 2));
    println!("{:?}", from_decimal(&[4, 2, 3], 16));
    println!("{:?}", to_decimal(&[0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0], 2));
    println!("{:?}", to_decimal(&[1, 2, 1, 0, 1, 0], 2));

    assert_eq!(convert(&[0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0], 2, 16), Ok(vec![3, 10, 6]));
    assert_eq!(convert(&[3, 46, 60], 97, 73), Ok(vec![6, 10, 45]));
    assert_eq!(convert(&[2, 10], 16, 3), Ok(vec![1, 1, 2, 0]));
    assert_eq!(convert(&[0], 10, 2), Ok(vec![0]));
    assert_eq!(convert(&[0, 0, 0], 10, 2), Ok(vec![0]));
    assert_eq!(convert(&[9, 15, 2], 16, 2), Ok(vec![1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0]));
    assert_eq!(convert(&[1, 2, 1, 0, 1, 0], 2, 10), Err(InvalidDigit(2)));

}