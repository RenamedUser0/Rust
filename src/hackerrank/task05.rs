pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &apple in apples {
        let pos = a + apple;
        if pos >= s && pos <= t {
            apple_count += 1;
        }
    }

    for &orange in oranges {
        let pos = b + orange;
        if pos >= s && pos <= t {
            orange_count += 1;
        }
    }

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;

        let apples = [-2, 2, 1];
        let oranges = [5, -6];

        let result = count_apples_and_oranges(s, t, a, b, &apples, &oranges);

        assert_eq!(result, (1, 1));
    }
}
