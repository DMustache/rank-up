struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;

        for bracket in s.chars() {
            match bracket {
                '(' => stack.push(')'),
                ')' => {
                    if stack.pop() != Some(bracket) {
                        result += 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        result + stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    }

    #[test]
    fn already_valid_and_unmatched_parentheses() {
        for (input, expected) in [
            ("()", 0),
            ("(())", 0),
            (")(", 2),
            ("()))((", 4),
            ("(()))(", 2),
        ] {
            assert_eq!(
                Solution::min_add_to_make_valid(input.to_string()),
                expected,
                "{input}"
            );
        }
    }
}
