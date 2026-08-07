struct Solution;

impl Solution {
    fn digit_power(digit: u8) -> [u8; 4] {
        match digit {
            2 => [1, 0, 0, 0],
            3 => [0, 1, 0, 0],
            4 => [2, 0, 0, 0],
            5 => [0, 0, 1, 0],
            6 => [1, 1, 0, 0],
            7 => [0, 0, 0, 1],
            8 => [3, 0, 0, 0],
            9 => [0, 2, 0, 0],
            _ => [0, 0, 0, 0],
        }
    }

    fn remove_digit_power(mut required: [u8; 4], digit: u8) -> [u8; 4] {
        let power = Self::digit_power(digit);

        for i in 0..required.len() {
            required[i] = required[i].saturating_sub(power[i]);
        }

        required
    }

    fn min_digits_for_two_three(two: u8, three: u8) -> usize {
        let mut result = usize::MAX;
        let max_sixes = two.min(three);

        for sixes in 0..=max_sixes {
            let remaining_two = two - sixes;
            let remaining_three = three - sixes;
            let count = sixes as usize
                + remaining_two.div_ceil(3) as usize
                + remaining_three.div_ceil(2) as usize;

            result = result.min(count);
        }

        result
    }

    fn min_digits(required: [u8; 4]) -> usize {
        required[2] as usize
            + required[3] as usize
            + Self::min_digits_for_two_three(required[0], required[1])
    }

    fn can_fill(required: [u8; 4], remaining_digits: usize) -> bool {
        Self::min_digits(required) <= remaining_digits
    }

    fn get_power(&self, number: &mut i64, divisor: i64) -> Result<u8, &str> {
        if *number == 0i64 {
            return Err("zero");
        }
        if divisor < 2 {
            return Err("one");
        }
        if *number < divisor {
            return Err("divisor must be greater");
        }

        let mut power = 0;

        while *number % divisor == 0 {
            *number /= divisor;
            power += 1;
        }

        Ok(power)
    }

    fn get_divisors(&self, mut number: i64) -> Vec<(i64, u8)> {
        let two = 2;
        let three = 3;
        let four = 4;
        let five = 5;
        let six = 6;

        if number <= 1 {
            return Vec::new();
        }

        if number < four {
            return vec![(number, 1)];
        }

        let mut result = Vec::new();

        if number % two == 0 {
            result.push((two, self.get_power(&mut number, two).unwrap()));
        }
        if number % three == 0 {
            result.push((three, self.get_power(&mut number, three).unwrap()));
        }

        let mut divisor = five;

        while divisor * divisor <= number {
            if number % divisor == 0 {
                result.push((divisor, self.get_power(&mut number, divisor).unwrap()));
            }
            if number % (divisor + two) == 0 {
                result.push((
                    divisor + two,
                    self.get_power(&mut number, divisor + two).unwrap(),
                ));
            }

            divisor += six;
        }

        if number != 1 {
            result.push((number, 1));
        }

        return result;
    }

    fn build_min_number(
        len: usize,
        mut required: [u8; 4],
        lower_bound: Option<&[u8]>,
    ) -> Option<String> {
        let mut result = String::with_capacity(len);
        let mut tight = lower_bound.is_some();

        for index in 0..len {
            let start = if tight {
                (lower_bound.unwrap()[index] - b'0').max(1)
            } else {
                1
            };
            let remaining = len - index - 1;
            let mut selected = None;

            for digit in start..=9 {
                let next_required = Self::remove_digit_power(required, digit);

                if Self::can_fill(next_required, remaining) {
                    selected = Some((digit, next_required));
                    break;
                }
            }

            if let Some((digit, next_required)) = selected {
                result.push((b'0' + digit) as char);
                required = next_required;

                if let Some(bound) = lower_bound {
                    tight = tight && digit == bound[index] - b'0';
                }
            } else {
                return None;
            }
        }

        if required == [0, 0, 0, 0] {
            Some(result)
        } else {
            None
        }
    }

    fn build_same_length_number(num: &[u8], required: [u8; 4]) -> Option<String> {
        let mut prefix_required = Vec::with_capacity(num.len() + 1);
        let mut prefix_valid = Vec::with_capacity(num.len() + 1);

        prefix_required.push(required);
        prefix_valid.push(true);

        for &digit_byte in num {
            let previous_required = *prefix_required.last().unwrap();
            let valid = *prefix_valid.last().unwrap() && digit_byte != b'0';
            let next_required = if valid {
                Self::remove_digit_power(previous_required, digit_byte - b'0')
            } else {
                previous_required
            };

            prefix_required.push(next_required);
            prefix_valid.push(valid);
        }

        if *prefix_valid.last().unwrap() && *prefix_required.last().unwrap() == [0, 0, 0, 0] {
            return String::from_utf8(num.to_vec()).ok();
        }

        for index in (0..num.len()).rev() {
            if !prefix_valid[index] {
                continue;
            }

            let current_digit = num[index] - b'0';
            let suffix_len = num.len() - index - 1;

            for digit in (current_digit + 1).max(1)..=9 {
                let next_required = Self::remove_digit_power(prefix_required[index], digit);

                if !Self::can_fill(next_required, suffix_len) {
                    continue;
                }

                let suffix = Self::build_min_number(suffix_len, next_required, None)?;
                let mut result = String::from_utf8(num[..index].to_vec()).ok()?;

                result.push((b'0' + digit) as char);
                result.push_str(&suffix);

                return Some(result);
            }
        }

        None
    }

    pub fn smallest_number(num: String, t: i64) -> String {
        let t_divisors = Self.get_divisors(t);
        let mut required = [0; 4];

        for (divisor, power) in t_divisors {
            match divisor {
                2 => required[0] = power,
                3 => required[1] = power,
                5 => required[2] = power,
                7 => required[3] = power,
                _ => return "-1".to_string(),
            }
        }

        let num_bytes = num.as_bytes();

        if let Some(result) = Self::build_same_length_number(num_bytes, required) {
            return result;
        }

        let len = (num_bytes.len() + 1).max(Self::min_digits(required));

        Self::build_min_number(len, required, None).unwrap_or_else(|| "-1".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            "1488".to_string(),
            Solution::smallest_number("1234".to_string(), 256)
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            "12355".to_string(),
            Solution::smallest_number("12355".to_string(), 50)
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            "-1".to_string(),
            Solution::smallest_number("11111".to_string(), 26)
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            "21".to_string(),
            Solution::smallest_number("19".to_string(), 2)
        )
    }
}
