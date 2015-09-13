use std::collections::HashMap;

pub struct School {
    pub registrar: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School { registrar: HashMap::<u32, Vec<String>>::new() }
    }

    pub fn add(&mut self, grade: u32, student_name: &str) {
        let mut students = self.registrar
            .entry(grade)
            .or_insert(vec![]);
        students.push(student_name.to_string());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades : Vec<u32> = self.registrar.keys()
            .cloned()
            .collect();
        grades.sort();
        grades
    }

    pub fn grade<'a>(&'a self, grade: u32) -> Option<&'a Vec<String>> {
        self.registrar.get(&grade)
    }
}
