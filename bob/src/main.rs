pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_alphabetic = message.chars().any(|c| c.is_alphabetic());
    let is_uppercase = (message == message.to_uppercase()) && is_alphabetic;

    if message.is_empty() {
        return "Fine. Be that way!";
    }
    if is_uppercase && (message.ends_with('?')) {
        return "Calm down, I know what I'm doing!";
    }
    if is_uppercase {
        return "Whoa, chill out!";
    }
    if message.ends_with('?') {
        return "Sure.";
    }

    "Whatever."
}

fn main() {
    println!("{}", reply(""));
    println!("{}", reply("WHAT"));
    println!("{}", reply("WHAT?"));
    println!("{}", reply("What?"));
    println!("{}", reply("Fuck."));
    println!("{}", reply(":) ?"));
    println!("{}", reply("1, 2, 3"));
    println!("{}", reply("4?"));
}
