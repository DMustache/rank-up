struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for bracket in s.chars() {
            match bracket {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(bracket) {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = "()".to_string();

        assert_eq!(true, Solution::is_valid(case));
    }

    #[test]
    fn example2() {
        let case = "()[]{}".to_string();

        assert_eq!(true, Solution::is_valid(case));
    }

    #[test]
    fn example3() {
        let case = "(]".to_string();

        assert_eq!(false, Solution::is_valid(case));
    }

    #[test]
    fn example4() {
        let case = "([)]".to_string();
        assert_eq!(false, Solution::is_valid(case))
    }
}
