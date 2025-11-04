use meal_card_manager::{
    student::{Student, StudentStatus},
    types::{Amount, MealCard},
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
