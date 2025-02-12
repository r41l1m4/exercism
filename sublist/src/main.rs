
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    if first_list.len() >= second_list.len() {
        if second_list.is_empty() {
            return Comparison::Superlist;
        }
        let indexes = match get_indexes_in_bigger(second_list[0], first_list) {
            Some(indexes) => indexes,
            None => return Comparison::Unequal,
        };
        if is_sublist(second_list, first_list, indexes) {
            if first_list.len() == second_list.len() {
                return Comparison::Equal
            }
            return Comparison::Superlist
        }
    }else if second_list.len() > first_list.len() {
        if first_list.is_empty() {
            return Comparison::Sublist;
        }
        let indexes = match get_indexes_in_bigger(first_list[0], second_list) {
            Some(indexes) => indexes,
            None => return Comparison::Unequal,
        };
        if is_sublist(first_list, second_list, indexes) {
            return Comparison::Sublist
        }
    }
    Comparison::Unequal
}

fn get_indexes_in_bigger(first_smaller: i32, bigger: &[i32]) -> Option<Vec<usize>> {
    let mut indexes = vec![];

    for (bigger_index, item) in bigger.iter().enumerate() {
        if *item == first_smaller {
            indexes.push(bigger_index);
        }
    }

    match indexes.is_empty() {
        false => Some(indexes),
        true => None
    }
}

fn is_sublist(smaller: &[i32], bigger: &[i32], indexes: Vec<usize>) -> bool {
    let mut is_it = false;
    let mut found = false;

    if smaller.is_empty() {
        return true;
    }

    for index in indexes {
        if (bigger.len() - index) < smaller.len() {
            break;
        }
        for smaller_index in 0..smaller.len() {
            if bigger[index + smaller_index] == smaller[smaller_index] {
               is_it = true;
            }else {
                is_it = false;
                break;
            }
            if smaller_index == smaller.len() - 1 {
                found = true;
            }
        }
        if found {
            break;
        }
    }

    is_it && found
}

fn main() {
    let first_list = vec![];
    let second_list: [i32; 0] = [];
    println!("{:?}", sublist(&first_list, &second_list));

    let first_list = vec![1, 2, 3, 4];
    let second_list = vec![1, 2, 3];
    println!("{:?}", sublist(&first_list, &second_list));

    let first_list = vec![0, 1, 2, 3, 4];
    let second_list = vec![1, 2, 4];
    println!("{:?}", sublist(&first_list, &second_list));

    let first_list = vec![1, 2, 3];
    let second_list = vec![0, 1, 2, 3, 4];
    println!("{:?}", sublist(&first_list, &second_list));

    let first_list = vec![1, 2, 3];
    let second_list = vec![1, 2, 3];
    println!("{:?}", sublist(&first_list, &second_list));

    let list = vec![0, 1, 2, 3, 1, 2, 5, 6];
    let smaller = vec![1, 2, 5];
    let element = 1;
    println!("{:?}", get_indexes_in_bigger(element, &list));
    let indexes = get_indexes_in_bigger(element, &list).unwrap();
    println!("{}", is_sublist(&smaller, &list, indexes));

}
