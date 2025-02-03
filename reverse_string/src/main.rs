pub fn reverse(input: &str) -> String {
    let mut vec: Vec<char> = vec![];
    for char in input.chars() {
        vec.push(char);
    }
    vec.reverse();
    let mut str1 = String::new();
    for i in vec {
        str1.push(i);
    }
    str1
}


fn main() {
    println!("{}", reverse("Arara"));
}