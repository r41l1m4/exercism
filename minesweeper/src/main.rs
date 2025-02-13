pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![]
    }

    remove_borders(
        parse_minefield(
            add_borders(
                minefield
            )
        )
    )
}

fn add_borders(minefield: &[&str]) -> Vec<Vec<String>> {
    let new_line_length = minefield[0].len() + 2;
    let filler_line = vec![0.to_string(); new_line_length];
    let mut result = vec![];
    result.push(filler_line.clone());

    for line in minefield {
        let char_vec = line.chars();
        let mut string_vec: Vec<String> = char_vec.into_iter()
                                  .map(|ch| ch.to_string())
                                  .collect();
        string_vec.insert(0, 0.to_string());
        string_vec.push(0.to_string());

        result.push(string_vec);
    }
    result.push(filler_line);


    result
}

fn remove_borders(minefield_with_borders: Vec<Vec<String>>) -> Vec<String> {
    let mut minefield = minefield_with_borders;
    minefield.remove(minefield.len() - 1);
    minefield.remove(0);
    let mut result = vec![];

    for item in minefield {
        let without_filling: Vec<String> = item.into_iter()
                                                .filter(|ch| ch != "0")
                                                .collect();
        let new_string = String::from_iter(without_filling);
        result.push(new_string);
    }

    result
}

fn parse_minefield(minefield_with_borders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result = minefield_with_borders.clone();

    for idx_lines in 1..(minefield_with_borders.len() - 1) {
        for idx_columns in 1..(minefield_with_borders[0].len() - 1) {
            let mut num_mines = 0;

            if minefield_with_borders[idx_lines][idx_columns] == " " {
                for idx_line_sublist in idx_lines-1..=idx_lines+1 {
                    for idx_column_sublist in idx_columns-1..=idx_columns+1 {
                        if minefield_with_borders[idx_line_sublist][idx_column_sublist] == "*" {
                            num_mines += 1;
                        }
                    }
                }
            }
            if num_mines != 0 {
                result[idx_lines][idx_columns] = num_mines.to_string();
            }
        }
    }

    result
}

fn main() {
    let test = &["***",
                        "* *",
                        "** "];
    let _test2 = &["   ",
                    " * ",
                    "   ",];
    let _test3 = &[" * * "];
    println!("{:?}", &test);
    let test_with_borders = add_borders(test);
    println!("{:?}", &test_with_borders);

    let parsed_minefield = parse_minefield(test_with_borders.clone());
    println!("{:?}", &parsed_minefield);

    let test_as_original = remove_borders(parsed_minefield);
    println!("{:?}", &test_as_original);

    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let input = &["*   *"];
    let expected = &["*1 1*"];
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",]);
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let input = &[];
    let expected: &[&str] = &[];
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let input = &[""];
    let expected = &[""];
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ", ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);

    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
