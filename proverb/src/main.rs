pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    for index in 0..(list.len()) {
        if index == list.len() - 1 {
            result.push_str(format!("And all for the want of a {}.", list[0]).as_str());
            break;
        }
        result.push_str(format!("For want of a {} the {} was lost.\n", list[index], list[index + 1]).as_str())
    }
    result
}

fn main() {
    let wordlist = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];
    println!("{}", build_proverb(&wordlist));
}