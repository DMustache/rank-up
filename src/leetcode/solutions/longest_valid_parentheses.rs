struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut invalid_parenthesis_indexes = Vec::new();

        for (idx, bracket) in s.bytes().enumerate() {
            match bracket {
                b'(' => stack.push(idx),
                b')' => {
                    if stack.pop().is_none() {
                        invalid_parenthesis_indexes.push(idx);
                    }
                }
                _ => unreachable!(),
            }
        }

        invalid_parenthesis_indexes.extend(stack);
        invalid_parenthesis_indexes.sort_unstable();

        let mut start = 0;
        let mut longest = 0;
        for idx in invalid_parenthesis_indexes {
            longest = longest.max(idx - start);
            start = idx + 1;
        }
        longest.max(s.len() - start) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn valid_spans_and_unmatched_parentheses() {
        for (input, expected) in [
            ("()()", 4),
            ("()(())", 6),
            ("()(()", 2),
            ("())(()", 2),
            ("(((", 0),
            (")))", 0),
        ] {
            assert_eq!(
                Solution::longest_valid_parentheses(input.to_string()),
                expected,
                "{input}"
            );
        }
    }
}
