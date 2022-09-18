use crate::domain::entities::{Student, StudentName, StudentId, StudentColors};

pub enum Insert {
    Ok(StudentId),
    Conflict,
    Error,
}

pub trait Repository {
    fn insert(&mut self, number: StudentId, name: StudentName, colors: StudentColors) -> Insert;
}

pub struct InMemoryRepository {
    error: bool,
    students: Vec<Student>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let students: Vec<Student> = vec![];
        Self {
            error: false,
            students,
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert (&mut self, number: StudentId, name: StudentName, colors: StudentColors) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.students.iter().any(|student| student.number == number) {
            return Insert::Conflict;
        }

        let number_clone = number.clone();
        self.students.push(Student::new(number_clone, name, colors));
        Insert::Ok(number)
    }
}