pub fn get_nth_fibonacci_num(n: i32) -> i32 {
    let mut arr: [i32; 3] = [0, 1, 1];

    if n < 2 {
        return arr[n as usize];
    }

    let mut count = 2;
    while count < n {
        let sum = arr[1] + arr[2];
        arr[1] = arr[2];
        arr[2] = sum;

        count += 1;
    }

    return arr[2];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_if_nth_fibonacci_num_is_correct() {
        assert_eq!(get_nth_fibonacci_num(1), 1);
        assert_eq!(get_nth_fibonacci_num(2), 1);
        assert_eq!(get_nth_fibonacci_num(3), 2);
        assert_eq!(get_nth_fibonacci_num(20), 6765);
    }
}