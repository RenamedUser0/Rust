pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len() as i32;
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i as usize][i as usize];
        secondary += arr[i as usize][(n - 1 - i) as usize];
    }

    (primary - secondary).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_1() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(arr), 15);
    }

    #[test]
    fn test_diagonal_difference_2() {
        let arr = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];
        assert_eq!(diagonal_difference(arr), 2);
    }
}
