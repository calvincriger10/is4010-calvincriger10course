mod student;
use student::{CourseGrade, Grade, Student, StudentDatabase};

fn main() {
    let mut db = StudentDatabase::new();

    let mut s = Student::new(
        "S001".to_string(),
        "Alice".to_string(),
        "a@email.com".to_string(),
    );

    s.add_grade(CourseGrade::new(
        "IS4010".to_string(),
        "App Dev".to_string(),
        3,
        Grade::A,
    ));

    db.add_student(s).unwrap();

    println!("Students: {}", db.student_count());
}
