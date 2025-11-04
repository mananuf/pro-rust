use meal_card_manager::{
    errors::{CampusMealSystemError, TransactionError},
    student::StudentStatus,
    types::{Amount, CampusMealSystem, TransactionType},
};

#[test]
fn test_student_registration_will_pass_and_student_id_updated_appropraitely() {
    let mut manager = CampusMealSystem::new();
    let student_1_id = manager.register_student("mano", 200);

    println!("STUDENT 1 {}", &student_1_id);
    assert_eq!(student_1_id, 1);
    assert_eq!(manager.get_student(student_1_id).unwrap().level, 200);
    assert_eq!(
        manager.get_student(student_1_id).unwrap().name,
        "mano".to_string()
    );
    assert_eq!(
        manager.get_student(student_1_id).unwrap().meal_card.balance,
        Amount(0)
    );
    assert_eq!(
        manager.get_student(student_1_id).unwrap().status,
        StudentStatus::Active
    );

    let student_2_id = manager.register_student("banks", 500);

    assert_eq!(student_2_id, 2);

    println!("STUDENT 2 {}", &student_2_id);
    assert_eq!(manager.get_student(student_2_id).unwrap().level, 500);
    assert_eq!(
        manager.get_student(student_2_id).unwrap().name,
        "banks".to_string()
    );
    assert_eq!(
        manager.get_student(student_2_id).unwrap().meal_card.balance,
        Amount(0)
    );
    assert_eq!(
        manager.get_student(student_2_id).unwrap().status,
        StudentStatus::Active
    );
}

#[test]
fn test_suspend_student_will_work_if_student_exists() {
    let mut manager = CampusMealSystem::new();
    let student_1_id = manager.register_student("mano", 200);

    assert_eq!(student_1_id, 1);
    assert_eq!(
        manager.get_student(student_1_id).unwrap().status,
        StudentStatus::Active
    );

    let _ = manager.suspend_student(student_1_id, "Malpractice");
    assert_eq!(
        manager.get_student(student_1_id).unwrap().status,
        StudentStatus::Suspended("Malpractice".to_string())
    );
    assert_ne!(
        manager.get_student(student_1_id).unwrap().status,
        StudentStatus::Suspended("malpractice".to_string())
    );
}

#[test]
fn test_suspend_student_will_return_error_if_student_doesnt_exists() {
    let mut manager = CampusMealSystem::new();
    let student_100_id = 100;

    let response = manager.suspend_student(student_100_id, "Malpractice");

    assert!(response.is_err());
    let err_msg = response.err().unwrap();
    assert_eq!(
        err_msg,
        CampusMealSystemError::StudentIdError(format!(
            "student with ID: {student_100_id} does not exist in this collection"
        ))
    );
}

#[test]
fn test_credit_and_debit_transactions() {
    let mut manager = CampusMealSystem::new();
    let id = manager.register_student("alice", 100);

    let bal = manager
        .process_transaction(id, TransactionType::Credit(200))
        .expect("credit should succeed");
    assert_eq!(bal, 200);

    let bal = manager
        .process_transaction(id, TransactionType::Debit(50))
        .expect("debit should succeed");
    assert_eq!(bal, 150);

    // debit too much -> insufficient funds
    let result = manager.process_transaction(id, TransactionType::Debit(1000));
    assert!(matches!(
        result,
        Err(TransactionError::InsufficientFunds(_))
    ));
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        TransactionError::InsufficientFunds(1000)
    );
}

#[test]
fn test_transactions_on_suspended_account_fail() {
    let mut manager = CampusMealSystem::new();
    let id = manager.register_student("tom", 100);
    let _ = manager.suspend_student(id, "fraud");

    let res = manager.process_transaction(id, TransactionType::Credit(100));
    assert!(matches!(res, Err(TransactionError::SuspendedAccount(_))));
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        TransactionError::SuspendedAccount("fraud".to_string())
    );
}

#[test]
fn test_student_not_found_transaction() {
    let mut manager = CampusMealSystem::new();
    let student_id = 9999;
    let res = manager.process_transaction(student_id, TransactionType::Credit(10));
    assert!(matches!(res, Err(TransactionError::StudentNotFound(_))));
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        TransactionError::StudentNotFound(format!("Student {student_id} does not exist"))
    );
}
