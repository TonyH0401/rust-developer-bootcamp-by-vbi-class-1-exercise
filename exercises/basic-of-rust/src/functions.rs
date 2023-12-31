// Exercise 1
// Fix all errors
fn sum(x: i32, y: i32) -> i32 {
    let _result: i32;
    _result = x + y;
    _result
}

//Exercise 2
// Input: Provide an arbitrary value of n
// Implement sum function: 1+2+3+..n
// Output: Calculate sum 1 to n
pub fn sum_one_to_n(n: u32) -> u32 {
    let mut _result: u32 = 0;
    for _index in 1..n + 1 {
        _result = _result + _index;
    }
    _result
}

// Exercise 3
// Input: list of arbitrary numbers
// Problem: Calculate the average of a list of numbers
// Output: Average Number
fn calculate_average(numbers: &[f64]) -> f64 {
    // todo!()
    if numbers.len() == 0 {
        return 0.0;
    }
    let mut _result: f64 = 0.0;
    for _value in numbers {
        _result = _result + _value;
    }
    _result = _result / numbers.len() as f64;
    _result
}

// Exercise 4
// Calculate the sum of all even numbers in a list
fn sum_even_numbers(numbers: &[i32]) -> i32 {
    // todo!()
    let mut _result: i32 = 0;
    for _value in numbers {
        if _value % 2 == 0 {
            _result = _result + _value;
        }
    }
    _result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn sum_should_work() {
        let (x, y) = (1, 2);
        let s = sum(x, y);

        assert_eq!(s, 3);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_0() {
        let result = sum_one_to_n(0);

        assert_eq!(result, 0);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_1() {
        let result = sum_one_to_n(1);

        assert_eq!(result, 1);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_100() {
        let result = sum_one_to_n(100);

        assert_eq!(result, 5050);
    }

    // Test for exercise 3
    #[test]
    fn test_calculate_average() {
        // Test case 1: Non-empty slice
        let numbers = [2.5, 4.8, 6.3, 1.7, 3.9];
        let result = calculate_average(&numbers);
        assert_eq!(result, 3.84);
    }

    // Test for exercise 3
    #[test]
    fn test_calculate_average_empty() {
        // Test case 1: Non-empty slice
        let numbers = [];
        let result = calculate_average(&numbers);
        assert_eq!(result, 0.0);
    }

    // Test for exercise 4
    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(sum_even_numbers(&[10, 20, 30, 40, 50]), 150);
        assert_eq!(sum_even_numbers(&[15, 25, 35, 45, 55]), 0);
        assert_eq!(sum_even_numbers(&[-2, 0, 2, 4, 6]), 10);
    }
}
