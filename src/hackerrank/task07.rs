pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a / gcd(a, b) * b
    }

    let lcm_a = a.iter().copied().reduce(lcm).unwrap();
    let gcd_b = b.iter().copied().reduce(gcd).unwrap();

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}
