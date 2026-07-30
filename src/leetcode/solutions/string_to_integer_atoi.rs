struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s: Vec<char> = s.trim_start().chars().collect();

        if s.is_empty() {
            return 0;
        }

        let mut flip: i32 = 1;
        let mut pointer = 0;

        if s[0] == '-' {
            flip = -1;
            pointer = 1;
        } else if s[0] == '+' {
            pointer = 1;
        }

        let mut result: i32 = 0;

        while pointer < s.len() && s[pointer].is_ascii_digit() {
            let digit = s[pointer].to_digit(10).unwrap() as i32;

            result = match result
                .checked_mul(10)
                .and_then(|value| value.checked_add(digit))
            {
                Some(value) => value,
                None => {
                    if flip.is_negative() {
                        return i32::MIN;
                    } else {
                        return i32::MAX;
                    }
                }
            };

            pointer += 1;
        }

        result * flip
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let case = "42".to_string();
        assert_eq!(42, Solution::my_atoi(case));
    }

    #[test]
    fn example2() {
        let case = " -042".to_string();
        assert_eq!(-42, Solution::my_atoi(case));
    }

    #[test]
    fn example3() {
        let case = "1337c0d3".to_string();
        assert_eq!(1337, Solution::my_atoi(case));
    }

    #[test]
    fn example4() {
        let case = "-91283472332".to_string();
        assert_eq!(-2147483648, Solution::my_atoi(case.clone()));
    }

    #[test]
    fn example5() {
        let case = "+1".to_string();
        assert_eq!(1, Solution::my_atoi(case));
    }
}
