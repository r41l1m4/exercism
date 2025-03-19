use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let Ok(counter) = nucleotide_counts(dna) else { return Err('X') };
    if counter.is_empty() { return Ok(0) }

    let Some(num) = counter.get(&nucleotide) else {return Err('X')};
    Ok(*num)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counter: HashMap<char, usize> = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0), ]);
    let mut err = false;

    dna.chars()
        .for_each(|nucleotide| {
            match nucleotide {
                'A' => { counter.entry('A').and_modify(|count| *count += 1); },
                'C' => { counter.entry('C').and_modify(|count| *count += 1); },
                'G' => { counter.entry('G').and_modify(|count| *count += 1); },
                'T' => { counter.entry('T').and_modify(|count| *count += 1); },
                _ => err = true,
            }
        });
    
    if err { return Err('X') }
    Ok(counter)
}

fn main() {
    let dna = "GATTACA";
    dbg!(nucleotide_counts(dna).unwrap());
    dbg!(count('A',dna).unwrap());

    let dna = "";
    dbg!(nucleotide_counts(dna).unwrap());
    dbg!(count('A',dna).unwrap());

}