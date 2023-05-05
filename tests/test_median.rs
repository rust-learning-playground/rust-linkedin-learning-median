use median;

#[test]
fn median_odd() {
    let numbers = vec![5.0, 10.0, 15.0];
    assert_eq!(median::median(numbers), Some(10.0));
}
#[test]
fn median_even() {
    let numbers = vec![5.0, 10.0, 12.0, 15.0];
    assert_eq!(median::median(numbers), Some(11.0));
}
#[test]
fn median_empty() {
    let numbers = vec![];
    assert_eq!(median::median(numbers), None);
}
#[test]
fn median_unsorted() {
    let numbers = vec![5.0, 15.0, 12.0, 10.0];
    assert_eq!(median::median(numbers), Some(11.0));
}
