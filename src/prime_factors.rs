use crate::prime_numbers;

pub fn find_prime_factors(n: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();

    for i in 1..n {
        if n % i == 0 && prime_numbers::is_prime_number(i) {
            factors.push(i);
        }
    }

    return factors;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_if_find_prime_factors_returns_correct_values() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(5);
        assert_eq!(find_prime_factors(25), vec);
    }
}