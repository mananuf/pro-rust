use e07_enums::{MiniCalc, types::CalcState};

#[test]
fn test_full_pipeline() {
    let mut calc = MiniCalc::new();
    calc.input("1+2");
    calc.run();

    assert_eq!(calc.state, CalcState::Finished(3));
}

#[test]
fn test_invalid_expression() {
    let mut calc = MiniCalc::new();
    calc.input("++");
    calc.run();

    match calc.state {
        CalcState::Error(_) => assert!(true),
        _ => panic!("Expected error state"),
    }
}
