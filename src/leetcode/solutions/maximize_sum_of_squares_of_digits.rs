struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if num <= 0 || sum < 1 || sum > 9 * num {
            return String::new();
        }

        let mut numbers = vec![b'9'; num as usize];
        let difference = (9 * num - sum) as usize;
        let zero_digits = difference / 9;
        let remaining_reduction = (difference % 9) as u8;
        let zero_start = numbers.len() - zero_digits;

        numbers[zero_start..].fill(b'0');
        if remaining_reduction > 0 {
            numbers[zero_start - 1] -= remaining_reduction;
        }

        String::from_utf8(numbers).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_sum_of_squares(2, 3), "30".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_sum_of_squares(2, 17), "98".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_sum_of_squares(1, 10), "".to_string())
    }

    #[test]
    fn reduces_multiple_digits() {
        assert_eq!(Solution::max_sum_of_squares(5, 10), "91000");
    }
}
