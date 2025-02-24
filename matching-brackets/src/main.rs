pub fn brackets_are_balanced(string: &str) -> bool {
    let valid_chars = ['{', '[', '(', '}', ']', ')'];
    let mut queue: Vec<char> = vec![];
    let chars: Vec<char> = string.chars()
        .filter(|ch| valid_chars.contains(ch))
        .collect();

    for ch in chars {
        match ch {
            '{' => queue.push(ch),
            '[' => queue.push(ch),
            '(' => queue.push(ch),
            ch2  => {
                    let ch3 = queue.pop().unwrap_or(' ');
                    if (ch2 == '}') && (ch3  != '{') { return false };
                    if (ch2 == ']') && (ch3  != '[') { return false };
                    if (ch2 == ')') && (ch3  != '(') { return false };
                }
        }
    }
    queue.is_empty()
}

fn main() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    assert!(!brackets_are_balanced("[({]})"));
    assert!(!brackets_are_balanced("}{"));
    assert!(!brackets_are_balanced("{[)][]}"));
    assert!(!brackets_are_balanced("[({]})"));
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
    \\end{array}\\right)";
    assert!(brackets_are_balanced(input))
}