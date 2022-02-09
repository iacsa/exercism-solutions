use std::collections::HashMap;

pub struct School {
    map: HashMap<usize, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn grades(&self) -> Vec<usize> {
        let mut grades: Vec<_> = self.map.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: usize) -> Vec<String> {
        self.map.get(&grade).cloned().unwrap_or(vec![])
    }

    pub fn add(&mut self, grade: usize, name: &str) {
        let names = self.map.entry(grade).or_insert(vec![]);
        names.push(name.to_string());
        names.sort();
    }
}
