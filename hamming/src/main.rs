/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None }

    let differences = s1.chars().zip(s2.chars())
        .fold(0usize, |mut acc, (lt1, lt2)| {
            if lt1 != lt2 { acc += 1; }
            acc
        });

    Some(differences)
}

fn main() {
    let s1 = "GAGCCTACTAACGGGAT";
    let s2 = "CATCGTAATGACGGCCT";

    dbg!(hamming_distance(s1, s2).unwrap());
}