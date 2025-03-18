use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    h.iter()
        .for_each(|(score, letters)| {
            letters.iter()
                .for_each(|letter| { result.insert((*letter).to_ascii_lowercase(), *score); })
        });

    result
}

fn main() {
    let input = BTreeMap::from([(1, vec!['A', 'E', 'I', 'O', 'U'])]);
    let expected = BTreeMap::from([('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);
    assert_eq!(expected, transform(&input));
}