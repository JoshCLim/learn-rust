// can put integration tests in tests directory

#[test]
fn test_1() {
    use wk4_01_testing::my_add;
    assert_eq!(my_add(1, 2), 3);
}

#[test]
#[should_panic] // trait to check if a function panics
fn test_panic() {
    use wk4_01_testing::my_add;
    my_add(-1, 2);
}
