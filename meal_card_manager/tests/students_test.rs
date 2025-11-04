use meal_card_manager::{
    student::{Student, StudentStatus},
    types::{Amount, MealCard, TransactionType},
};

#[test]
fn test_student_creation_with_required_params_will_pass() {
    let student1 = Student::builder("mano", 200).build();

    assert_eq!(student1.name, "mano".to_string());
    assert_eq!(student1.level, 200);
    assert_eq!(student1.status, StudentStatus::Active);
    assert_eq!(student1.meal_card.balance, Amount(0));
}

#[test]
fn test_student_creation_with_required_params_and_status_will_pass() {
    let student1 = Student::builder("mano", 200)
        .status(StudentStatus::Suspended("invalid Behavior".to_string()))
        .build();

    assert_eq!(student1.name, "mano".to_string());
    assert_eq!(student1.level, 200);
    assert_eq!(
        student1.status,
        StudentStatus::Suspended("invalid Behavior".to_string())
    );
    assert_ne!(
        student1.status,
        StudentStatus::Suspended("Invalid Behavior".to_string())
    );
    assert_eq!(student1.meal_card.balance, Amount(0));
}

#[test]
fn test_student_creation_with_all_params_will_pass() {
    let meal_card = MealCard {
        balance: Amount(10),
    };
    let student1 = Student::builder("mano", 200)
        .status(StudentStatus::Suspended("invalid Behavior".to_string()))
        .meal_card(meal_card)
        .build();

    assert_eq!(student1.name, "mano".to_string());
    assert_eq!(student1.level, 200);
    assert_eq!(
        student1.status,
        StudentStatus::Suspended("invalid Behavior".to_string())
    );
    assert_ne!(
        student1.status,
        StudentStatus::Suspended("Invalid Behavior".to_string())
    );
    assert_ne!(student1.meal_card.balance, Amount(0));
    assert_eq!(student1.meal_card.balance, Amount(10));
}

#[test]
fn test_credit_transaction() {
    let mut s = Student::builder("mano", 200).build();

    let balance = s.apply_transaction(TransactionType::Credit(50)).unwrap();

    assert_eq!(balance, 50);
    assert_eq!(s.meal_card.balance, Amount(50));
}

#[test]
fn test_debit_transaction_fails_on_insufficient_funds() {
    let mut s = Student::builder("mano", 200).build();

    let result = s.apply_transaction(TransactionType::Debit(20));

    assert!(result.is_err());
}

#[test]
fn test_suspended_student_cannot_transact() {
    let mut s = Student::builder("mano", 200)
        .status(StudentStatus::Suspended("Fee default".into()))
        .build();

    let result = s.apply_transaction(TransactionType::Credit(100));

    assert!(result.is_err());
}
