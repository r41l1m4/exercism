pub fn verse(n: u32) -> String {
    match n {
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n\n", n, n, n-1),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n", n, n),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n\n", n, n, n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    (end..=start).rev()
        .for_each(|num| {
            result.push_str(verse(num).as_str())
        });
    result
}

fn main() {
    println!("{}", sing(3, 0));
}
