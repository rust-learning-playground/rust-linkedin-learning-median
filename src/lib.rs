pub fn median(mut l: Vec<f32>) -> Option<f32> {
    l.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if l.is_empty() {
        return None;
    }
    match l.len() % 2 {
        0 => return Some((l[l.len() / 2] + l[(l.len() / 2) - 1]) / 2.0),
        1 => return Some(l[l.len() / 2]),
        _ => return Some(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd() {
        let numbers = vec![5.0, 10.0, 15.0];
        assert_eq!(median(numbers), Some(10.0));
    }
    #[test]
    fn median_even() {
        let numbers = vec![5.0, 10.0, 12.0, 15.0];
        assert_eq!(median(numbers), Some(11.0));
    }
    #[test]
    fn median_empty() {
        let numbers = vec![];
        assert_eq!(median(numbers), None);
    }
    #[test]
    fn median_unsorted() {
        let numbers = vec![5.0, 15.0, 12.0, 10.0];
        assert_eq!(median(numbers), Some(11.0));
    }
}

