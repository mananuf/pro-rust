use std::collections::HashMap;

use crate::{
    errors::{CampusMealSystemError, TransactionError},
    student::{Student, StudentStatus},
};

type StudentId = u32;
type Students = HashMap<StudentId, Student>;

#[derive(Debug, Default, PartialEq)]
pub struct Amount(pub u64);

#[derive(Debug, Default)]
pub struct MealCard {
    pub balance: Amount,
}

#[derive(Debug, Default)]
pub struct CampusMealSystem {
    next_id: StudentId,
    pub students: Students,
}

impl CampusMealSystem {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            students: HashMap::new(),
        }
    }

    pub fn register_student(&mut self, name: &str, level: u32) -> StudentId {
        let new_student: Student = Student::builder(name, level).build();
        self.next_id += 1;
        let student_id = self.next_id;
        self.students.insert(student_id, new_student);

        student_id
    }

    pub fn suspend_student(
        &mut self,
        student_id: StudentId,
        reason: &str,
    ) -> Result<(), CampusMealSystemError> {
        if let Some(student) = self.students.get_mut(&student_id) {
            student.status = StudentStatus::Suspended(reason.to_string());
            return Ok(());
        }

        Err(CampusMealSystemError::StudentIdError(format!(
            "student with ID: {student_id} does not exist in this collection"
        )))
    }

    pub fn process_transaction(
        &mut self,
        id: StudentId,
        txn: TransactionType,
    ) -> Result<u64, TransactionError> {
        match self.students.get_mut(&id) {
            Some(student) => student.apply_transaction(txn),
            None => Err(TransactionError::StudentNotFound(format!(
                "Student {id} does not exist"
            ))),
        }

        // if let Some(student) = self.students.get_mut(&id) {
        //     match txn {
        //         TransactionType::Credit(amount) => {
        //             (*student).meal_card.balance = Amount((*student).meal_card.balance.0 + amount);
        //             (*student).meal_card.balance.0
        //         }
        //         TransactionType::Debit(amount) => {
        //             (*student).meal_card.balance = Amount((*student).meal_card.balance.0 - amount);
        //             (*student).meal_card.balance.0
        //         }
        //     };
        // }

        // Err(TransactionError::StudentNotFound(format!(
        //     "student with ID: {id} NOT FOUND"
        // )))
    }

    pub fn get_student(&self, student_id: StudentId) -> Option<&Student> {
        self.students.get(&student_id)
    }

    pub fn list_active_students(self) -> Students {
        self.students
            .into_iter()
            .filter(|s| s.1.status == StudentStatus::Active)
            .collect()
    }
}

pub enum TransactionType {
    Credit(u64), // add funds
    Debit(u64),  // remove funds
}
