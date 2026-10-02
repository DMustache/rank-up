struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        if n > 0 {
            let mut current = String::new();
            Self::parenthesis_inner(0, 0, n, &mut current, &mut result);
        }
        result
    }

    fn parenthesis_inner(
        open: i32,
        close: i32,
        n: i32,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if close == n {
            result.push(current.clone());
            return;
        }

        if open < n {
            current.push('(');
            Self::parenthesis_inner(open + 1, close, n, current, result);
            current.pop();
        }

        if close < open {
            current.push(')');
            Self::parenthesis_inner(open, close + 1, n, current, result);
            current.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::generate_parenthesis(1), ["()"]);
    }
}
