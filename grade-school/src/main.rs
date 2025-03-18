use std::collections::HashMap;

pub struct School {
    registry: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.registry.contains_key(student) {
            self.registry.insert(student.to_string(), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut values = vec![];
        self.registry.values().for_each(|value| {
            if !values.contains(value) { values.push(*value) }
        });
        values.sort();
        values
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut vec = self.registry.clone().into_iter()
            .filter_map(|(key, value)| {
                if value != grade { return None; }
                Some(key)
            })
            .collect::<Vec<String>>();

        vec.sort();
        vec
    }
}

fn main() {
    let mut hm = HashMap::new();
    hm.insert("James".to_string(), 2);
    hm.insert("Jane".to_string(), 3);
    hm.insert("Joan".to_string(), 3);

    if hm.get("James").is_none() {
        hm.insert("James".to_string(), 3);
    }

    dbg!(&hm);
    let mut values = vec![];
    let _ = &hm.values().for_each(|value| {
        if !values.contains(value) { values.push(*value) }
    });

    dbg!(&values);

    let grade = 2;
    let vec = hm.into_iter()
        .filter_map(|(key, value)| {
            if value != grade { return None; }
            Some(key)
        })
        .collect::<Vec<String>>();

    dbg!(&vec);
}