use std::collections::{BTreeMap, BinaryHeap};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grades_students: BTreeMap<u32, BinaryHeap<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades_students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades_students
            .entry(grade)
            .or_insert(BinaryHeap::new())
            .push(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades_students.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let value = self.grades_students.get(&grade);
        match value {
            Some(students) => students.clone().into_sorted_vec(),
            None => {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grade_school as school;
    fn to_owned(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }
    #[test]
    fn test_grades_for_empty_school() {
        let s = school::School::new();
        assert_eq!(s.grades(), vec![]);
    }
    #[test]
    fn test_grades_for_one_student() {
        let mut s = school::School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grades(), vec![2]);
    }
    #[test]
    fn test_grades_for_several_students_are_sorted() {
        let mut s = school::School::new();
        s.add(2, "Aimee");
        s.add(7, "Logan");
        s.add(4, "Blair");
        assert_eq!(s.grades(), vec![2, 4, 7]);
    }
    #[test]
    fn test_grades_when_several_students_have_the_same_grade() {
        let mut s = school::School::new();
        s.add(2, "Aimee");
        s.add(2, "Logan");
        s.add(2, "Blair");
        assert_eq!(s.grades(), vec![2]);
    }
    #[test]
    fn test_grade_for_empty_school() {
        let s = school::School::new();
        assert_eq!(s.grade(1), Vec::<String>::new());
    }
    #[test]
    fn test_grade_when_no_students_have_that_grade() {
        let mut s = school::School::new();
        s.add(7, "Logan");
        assert_eq!(s.grade(1), Vec::<String>::new());
    }
    #[test]
    fn test_grade_for_one_student() {
        let mut s = school::School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grade(2), to_owned(&["Aimee"]));
    }
    #[test]
    fn test_grade_returns_students_sorted_by_name() {
        let mut s = school::School::new();
        s.add(2, "James");
        s.add(2, "Blair");
        s.add(2, "Paul");
        assert_eq!(s.grade(2), to_owned(&["Blair", "James", "Paul"]));
    }
    #[test]
    fn test_add_students_to_different_grades() {
        let mut s = school::School::new();
        s.add(3, "Chelsea");
        s.add(7, "Logan");
        assert_eq!(s.grades(), vec![3, 7]);
        assert_eq!(s.grade(3), to_owned(&["Chelsea"]));
        assert_eq!(s.grade(7), to_owned(&["Logan"]));
    }
}
