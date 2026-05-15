pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> Option<i32> {
    let mut sum = 0;

    for (i, &cost) in bill.iter().enumerate() {
        if i != k {
            sum += cost;
        }
    }

    let actual = sum / 2;

    if actual == b {
        None
    } else {
        Some(b - actual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_shared() {
        let bill = vec![3, 10, 2, 9];
        let result = bon_appetit(&bill, 1, 12);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_bon_appetit_fair() {
        let bill = vec![3, 10, 2, 9];
        let result = bon_appetit(&bill, 1, 7);
        assert_eq!(result, None);
    }
}
