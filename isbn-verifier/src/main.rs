pub fn is_valid_isbn(isbn: &str) -> bool {
    is_isbn_valid(
        isbn_chars_to_digits_vec(
            &isbn_without_dashes(isbn)
        )
    )
}

fn isbn_without_dashes(isbn: &str) -> String {
    isbn.chars()
        .filter(|ch| ch != &'-')
        .collect::<String>()
}

fn isbn_chars_to_digits_vec(isbn: &str) -> Vec<u8> {
    if isbn.len() != 10 { return vec![] }

    let last_digit = match isbn.chars().last().unwrap() {
        'X' => "10",
        anything => {
            if !anything.is_numeric() { "" }
            else { &anything.to_string() }
        }
    };

    let mut digits = isbn[0..=8].chars()
        .filter(|ch| ch.is_numeric() )
        .map(|ch| {
            if ch == 'X' { return 10 }
            ch.to_digit(10).unwrap() as u8 })
        .collect::<Vec<u8>>();

    if !last_digit.is_empty() {
        digits.push(last_digit.parse().unwrap());
    }

    digits
}

fn is_isbn_valid(isbn: Vec<u8>) -> bool {
    if isbn.is_empty() || isbn.len() != 10 { return false }
    let checksum = isbn.into_iter().zip(1u8..=10)
        .fold(0, |mut acc, (isbn_digit, multiplier)| {
            acc += (isbn_digit * multiplier) as i32;
            acc
        });

    checksum % 11 == 0
}

fn main() {
    let isbn = "3-598-21508-8";
    let isbn2 = "3-598-21507-X";
    let isbn3 = "3-598-21507";
    let isbn4 = "3-598-21507-A";
    let isbn5 = "3598P215088";
    let isbn6 = "3-598-2X507-9";
    let isbn7 = "4-598-21507-B";

    dbg!(isbn_without_dashes(isbn));
    dbg!(isbn_without_dashes(isbn2));
    dbg!(isbn_without_dashes(isbn5));
    dbg!(isbn_without_dashes(isbn7));

    println!("{:?}", isbn_chars_to_digits_vec(&isbn_without_dashes(isbn)));
    println!("{:?}", isbn_chars_to_digits_vec(&isbn_without_dashes(isbn2)));
    println!("{:?}", isbn_chars_to_digits_vec(&isbn_without_dashes(isbn7)));

    println!("{}", is_valid_isbn(isbn));
    println!("{}", is_valid_isbn(isbn2));
    println!("{}", is_valid_isbn(isbn3));
    println!("{}", is_valid_isbn(isbn4));
    println!("{}", is_valid_isbn(isbn5));
    println!("{}", is_valid_isbn(isbn6));
    println!("{}", is_valid_isbn(isbn7));
}