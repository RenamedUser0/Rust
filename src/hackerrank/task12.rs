pub fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    let mut max_val = 0;
    let mut count = 0;

    for c in candles {
        if c > max_val {
            max_val = c;
            count = 1;
        } else if c == max_val {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(candles), 2);
    }

    #[test]
    fn test_case_2() {
        let candles = vec![1, 2, 3, 4, 4];
        assert_eq!(birthday_cake_candles(candles), 2);
    }

    #[test]
    fn test_case_3() {
        let candles = vec![5, 5, 5, 5];
        assert_eq!(birthday_cake_candles(candles), 4);
    }

    #[test]
    fn test_case_4() {
        let candles = vec![1];
        assert_eq!(birthday_cake_candles(candles), 1);
    }
}
