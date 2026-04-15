pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    if scores.is_empty() {
        return (0, 0);
    }

    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        } else if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), (2, 4));
    }

    #[test]
    fn test_single_element() {
        let scores = vec![100];
        assert_eq!(breaking_records(&scores), (0, 0));
    }

    #[test]
    fn test_increasing() {
        let scores = vec![1, 2, 3, 4, 5];
        assert_eq!(breaking_records(&scores), (4, 0));
    }

    #[test]
    fn test_decreasing() {
        let scores = vec![5, 4, 3, 2, 1];
        assert_eq!(breaking_records(&scores), (0, 4));
    }
}
