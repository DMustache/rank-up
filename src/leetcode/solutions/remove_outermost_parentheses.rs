struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut depth = 0;

        for ch in s.chars() {
            match ch {
                '(' => {
                    if depth > 0 {
                        result.push(ch);
                    }
                    depth += 1;
                }
                ')' => {
                    depth -= 1;
                    if depth > 0 {
                        result.push(ch);
                    }
                }
                _ => unreachable!(),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
