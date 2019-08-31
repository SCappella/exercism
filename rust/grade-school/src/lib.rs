use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    students: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students
            .entry(grade)
            .or_insert_with(Vec::new)
            .push(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students.get(&grade).cloned().map(|mut list| {
            list.sort_unstable();
            list
        })
    }
}
