pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let vec_diagram = diagram_into_vec(diagram);
    let Some(student_idx) = child_index(student) else { return vec![] };

    let student_idx = student_idx * 2;
    let mut child_plants: Vec<&'static str> = Vec::new();

    vec_diagram.into_iter()
        .for_each(|encoded_str| {
            encoded_str.into_iter()
                .skip(student_idx)
                .take(2)
                .for_each(|plant_code| {
                    let plant_name = Box::leak(encoding_to_plant(&plant_code).unwrap().into_boxed_str());
                    child_plants.push(plant_name)
            });
    });
    child_plants
}

fn encoding_to_plant(encoding: &str) -> Option<String> {
    match encoding {
        "G" => Some("grass".to_string()),
        "C" => Some("clover".to_string()),
        "R" => Some("radishes".to_string()),
        "V" => Some("violets".to_string()),
        _ => None,
    }
}

fn child_index(name: &str) -> Option<usize> {
    let children = vec!["Alice", "Bob", "Charlie", "David",
                        "Eve", "Fred", "Ginny", "Harriet",
                        "Ileana", "Joseph", "Kincaid", "Larry"];

    children.into_iter()
        .position(|child| child == name)
}

fn diagram_into_vec(diagram: &str) -> Vec<Vec<String>> {
    let diagram: Vec<&str> = diagram.split_whitespace().collect();
    let mut vec_diagram: Vec<Vec<String>> = vec![];

    for str in diagram {
        vec_diagram.push(str.chars().map(|ch| ch.to_string()).collect::<Vec<String>>());
    }

    vec_diagram
}

fn main() {
    let diagram = "RC
GG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);

    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "David";
    let expected = vec!["radishes", "violets", "clover", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}