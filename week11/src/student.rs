#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl Grade {
    pub fn to_gpa_points(&self) -> f32 {
        match self {
            Grade::A => 4.0,
            Grade::B => 3.0,
            Grade::C => 2.0,
            Grade::D => 1.0,
            Grade::F => 0.0,
        }
    }

    pub fn from_string(s: &str) -> Option<Grade> {
        match s.to_uppercase().as_str() {
            "A" => Some(Grade::A),
            "B" => Some(Grade::B),
            "C" => Some(Grade::C),
            "D" => Some(Grade::D),
            "F" => Some(Grade::F),
            _ => None,
        }
    }

    pub fn is_passing(&self) -> bool {
        match self {
            Grade::A | Grade::B | Grade::C => true,
            Grade::D | Grade::F => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CourseGrade {
    pub course_code: String,
    pub course_name: String,
    pub credits: u16,
    pub grade: Grade,
}

impl CourseGrade {
    pub fn new(code: String, name: String, credits: u16, grade: Grade) -> Self {
        Self {
            course_code: code,
            course_name: name,
            credits,
            grade,
        }
    }

    pub fn quality_points(&self) -> f32 {
        self.credits as f32 * self.grade.to_gpa_points()
    }
}

pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub credits_earned: u16,
    pub grades: Vec<CourseGrade>,
}

impl Student {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            credits_earned: 0,
            grades: Vec::new(),
        }
    }

    pub fn add_grade(&mut self, course: CourseGrade) {
        self.credits_earned += course.credits;
        self.grades.push(course);
    }

    pub fn calculate_gpa(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }

        let total_points: f32 = self.grades.iter().map(|c| c.quality_points()).sum();

        let total_credits: f32 = self.grades.iter().map(|c| c.credits as f32).sum();

        total_points / total_credits
    }

    pub fn class_standing(&self) -> &str {
        match self.credits_earned {
            0..=29 => "Freshman",
            30..=59 => "Sophomore",
            60..=89 => "Junior",
            _ => "Senior",
        }
    }

    pub fn can_graduate(&self) -> bool {
        self.credits_earned >= 120
    }
}

pub struct StudentDatabase {
    students: HashMap<String, Student>,
}

impl StudentDatabase {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            return Err("Student already exists".to_string());
        }

        self.students.insert(student.id.clone(), student);
        Ok(())
    }

    pub fn find_student(&self, id: &str) -> Option<&Student> {
        self.students.get(id)
    }

    pub fn student_count(&self) -> usize {
        self.students.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpa() {
        let mut s = Student::new("1".to_string(), "Test".to_string(), "x".to_string());

        s.add_grade(CourseGrade::new(
            "C".to_string(),
            "Class".to_string(),
            3,
            Grade::A,
        ));

        assert_eq!(s.calculate_gpa(), 4.0);
    }
}
