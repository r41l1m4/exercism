use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut indexes: Vec<usize> = (0..array.len()).collect();
    let mut array = array;
    if !array.is_sorted() {
        return None;
    }
    if array.len() <= 1{
        if !array.is_empty() && array.first().unwrap() == &key {
            return Some(0);
        }
        return None;
    }

    loop {
        if array.len() == 1 && array.first().unwrap() != &key {
            return None
        }
        let actual_idx = (array.len() - 1) / 2;
        match key.cmp(&array[actual_idx]) {
            Ordering::Equal => { return Some(indexes[actual_idx]) },
            Ordering::Greater => {
                let range = (array.len() / 2)..;
                array = &array[range.clone()];
                indexes = indexes[range.clone()].to_owned();
            },
            Ordering::Less => {
                let range = ..=(array.len() / 2) - 1;
                array = &array[range.clone()];
                indexes = indexes[range.clone()].to_owned();
            },
        }
    }
}

pub fn find_2(array: &[i32], key: i32) -> Option<usize> {
    if !array.is_sorted() { return None; }
    if array.len() <= 1 {
        return match !array.is_empty() && array.first()? == &key {
            true => Some(0),
            false => None,
        }
    }
    let mut indexed_pairs: Vec<(usize, &i32)> = (0..array.len()).zip(array).collect();

    loop {
        if indexed_pairs.len() == 1 && indexed_pairs.first()?.1 != &key {
            return None
        }
        let actual_idx = (indexed_pairs.len() - 1) / 2;
        match key.cmp(indexed_pairs[actual_idx].1) {
            Ordering::Equal => return Some(indexed_pairs[actual_idx].0),
            Ordering::Greater => indexed_pairs = indexed_pairs[(indexed_pairs.len() / 2)..].to_owned(),
            Ordering::Less => indexed_pairs = indexed_pairs[..=(indexed_pairs.len() / 2) - 1].to_owned(),
        }
    }
}

fn binary_search(list: &[i32], searched: i32) -> bool {
    if list.len() == 1 || list.is_empty(){
        if list.first().is_some() && list.first().unwrap() == &searched {
            return true;
        }
        return false;
    }
    if !list.is_sorted() { return false; }

    let actual = list[(list.len() - 1) / 2];
    if actual == searched {
        true
    }else if searched > actual {
        return binary_search(&list[list.len() / 2..], searched)
    }else if searched < actual{
        return binary_search(&list[..=(list.len() / 2) - 1], searched)
    }else {
        false
    }
}

fn main() {
    assert!(binary_search(&[1, 2, 3], 2));
    assert!(binary_search(&[1, 2, 3], 1));
    assert!(binary_search(&[1, 2, 3], 3));
    assert!(!binary_search(&[1, 2], 3));
    assert!(!binary_search(&[], 1));
    assert!(!binary_search(&[5, 2, 6], 2));

    assert_eq!(find(&[1, 2, 3], 2), Some(1));
    assert_eq!(find(&[1, 2, 3], 1), Some(0));
    assert_eq!(find(&[1, 2, 3], 3), Some(2));
    assert_eq!(find(&[], 1), None);
    assert_eq!(find(&[5, 2, 6], 2), None);
    assert_eq!(find(&[1, 2], 3), None);

    assert_eq!(find_2(&[1, 2, 3], 2), Some(1));
    assert_eq!(find_2(&[1, 2, 3], 1), Some(0));
    assert_eq!(find_2(&[1, 2, 3], 3), Some(2));
    assert_eq!(find_2(&[], 1), None);
    assert_eq!(find_2(&[5, 2, 6], 2), None);
    assert_eq!(find_2(&[1, 2], 3), None);

    let vec = &[(0, 3), (1, 3), (2, 4)];
    println!("{}", vec.is_sorted());
}