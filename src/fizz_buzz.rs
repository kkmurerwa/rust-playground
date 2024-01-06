use std::io;
use std::error::Error;

pub fn run_fizz_buzz() -> Vec<String> {
    // Take user input
    println!("Enter a number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().unwrap_or_else(|_| {
        println!("Could not parse number. Defaulting to -1");
        -1
    });

    let mut response: Vec<String> = Vec::new();

    for i in 1..=input {
        let string = get_output_string(&i);

        response.push(string);
    }

    println!("Output: {:?}", response);

    return response;
}

fn get_output_string(input: &i32) -> String {
    let mut output = String::new();

    if input % 3 == 0 && input % 5 == 0 {
        output.push_str("FizzBuzz")
    } else if input % 3 == 0 {
        output.push_str("Fizz");
    } else if input % 5 == 0 {
        output.push_str("Buzz")
    } else {
        output.push_str(&*input.to_string())
    }

    return output;
}

// Test run_fizz_buzz
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run_fizz_buzz() {
        let output = run_fizz_buzz();

        let mut expected: Vec<String> = Vec::new();
        expected.push("1".to_string());
        expected.push("2".to_string());
        expected.push("Fizz".to_string());
        expected.push("4".to_string());
        expected.push("Buzz".to_string());
        expected.push("Fizz".to_string());
        expected.push("7".to_string());
        expected.push("8".to_string());
        expected.push("Fizz".to_string());
        expected.push("Buzz".to_string());
        expected.push("11".to_string());
        expected.push("Fizz".to_string());
        expected.push("13".to_string());
        expected.push("14".to_string());
        expected.push("FizzBuzz".to_string());

        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_output_string() {
        assert_eq!(get_output_string(&3), "Fizz");
        assert_eq!(get_output_string(&5), "Buzz");
        assert_eq!(get_output_string(&15), "FizzBuzz");
        assert_eq!(get_output_string(&1), "1");
    }
}