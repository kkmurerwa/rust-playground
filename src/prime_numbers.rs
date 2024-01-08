pub fn find_nth_prime_number(n: i32) -> i32 {
    let mut count = 0;
    for i in 1..i32::MAX {
        if is_prime_number(i) {
            count += 1;

            if count == n { return i; }
        }
    }

    return 0;
}

pub(crate) fn is_prime_number(num: i32) -> bool {
    if num == 2i32 { return true }
    if num % 2 == 0 { return false }

    let p: i64 = 2;
    let pow: i64 = p.pow((num - 1) as u32);

    return pow % (num as i64) == 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_if_is_prime_number_returns_correct_result() {
        assert_eq!(is_prime_number(7), true);
        assert_eq!(is_prime_number(2), true);
        assert_eq!(is_prime_number(5), true);
        assert_eq!(is_prime_number(1), false);
        assert_eq!(is_prime_number(27), false);
        assert_eq!(is_prime_number(31), true);
    }

    #[test]
    fn test_check_if_find_nth_prime_number_returns_correct_value() {
        assert_eq!(find_nth_prime_number(1), 2);
        assert_eq!(find_nth_prime_number(2), 3);
        assert_eq!(find_nth_prime_number(3), 5);
        assert_eq!(find_nth_prime_number(4), 7);
        assert_eq!(find_nth_prime_number(5), 11);
        assert_eq!(find_nth_prime_number(6), 13);
        assert_eq!(find_nth_prime_number(7), 17);
        assert_eq!(find_nth_prime_number(8), 19);
    }
}