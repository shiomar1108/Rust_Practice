use std::collections::BTreeMap;

pub struct School {
    data: BTreeMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        Self {
            data: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.data.values().any(|students| students.iter().any(|s| s == student)) {
            let students = self.data.entry(grade).or_default();
            if !students.iter().any(|s| s == student) {
                students.push(student.to_string());
                students.sort();
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.data.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.data.get(&grade).cloned().unwrap_or_default()
    }
}
