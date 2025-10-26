use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    roaster: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.roaster.values().flatten().any(|s| s == student) {
            return;
        }

        self.roaster.entry(grade).or_default().push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roaster.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.roaster.get(&grade).cloned().unwrap_or_default();
        students.sort();
        students
    }
}
