use std::io;

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    staircase(n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let result = staircase(4);
        assert_eq!(result.len(), 4);
    }
}
