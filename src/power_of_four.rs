pub fn check_if_power_of_four(n: i32) -> bool {
    return n.count_ones() == 1 && n.trailing_zeros() % 2 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_power_of_four() {
        assert_eq!(check_if_power_of_four(16), true);
        assert_eq!(check_if_power_of_four(5), false);
        assert_eq!(check_if_power_of_four(1), true);
        assert_eq!(check_if_power_of_four(0), false);
        assert_eq!(check_if_power_of_four(67108864), true);
        assert_eq!(check_if_power_of_four(-16), false);
    }
}