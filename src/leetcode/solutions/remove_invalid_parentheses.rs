use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let (mut remove_open, mut remove_close) = (0, 0);

        for &ch in &chars {
            match ch {
                '(' => remove_open += 1,
                ')' if remove_open > 0 => remove_open -= 1,
                ')' => remove_close += 1,
                _ => {}
            }
        }

        fn search(
            chars: &[char],
            index: usize,
            balance: usize,
            remove_open: usize,
            remove_close: usize,
            current: &mut String,
            results: &mut HashSet<String>,
        ) {
            if chars.len() - index < remove_open + remove_close {
                return;
            }

            if index == chars.len() {
                if balance == 0 && remove_open == 0 && remove_close == 0 {
                    results.insert(current.clone());
                }
                return;
            }

            let ch = chars[index];
            match ch {
                '(' => {
                    if remove_open > 0 {
                        search(
                            chars,
                            index + 1,
                            balance,
                            remove_open - 1,
                            remove_close,
                            current,
                            results,
                        );
                    }
                    current.push(ch);
                    search(
                        chars,
                        index + 1,
                        balance + 1,
                        remove_open,
                        remove_close,
                        current,
                        results,
                    );
                    current.pop();
                }
                ')' => {
                    if remove_close > 0 {
                        search(
                            chars,
                            index + 1,
                            balance,
                            remove_open,
                            remove_close - 1,
                            current,
                            results,
                        );
                    }
                    if balance > 0 {
                        current.push(ch);
                        search(
                            chars,
                            index + 1,
                            balance - 1,
                            remove_open,
                            remove_close,
                            current,
                            results,
                        );
                        current.pop();
                    }
                }
                _ => {
                    current.push(ch);
                    search(
                        chars,
                        index + 1,
                        balance,
                        remove_open,
                        remove_close,
                        current,
                        results,
                    );
                    current.pop();
                }
            }
        }

        let mut results = HashSet::new();
        search(
            &chars,
            0,
            0,
            remove_open,
            remove_close,
            &mut String::new(),
            &mut results,
        );
        let mut results: Vec<String> = results.into_iter().collect();
        results.sort_unstable();
        results
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solutions::remove_invalid_parentheses::Solution;

    fn assert_output(input: &str, expected: &[&str]) {
        let mut response = Solution::remove_invalid_parentheses(input.to_owned());
        response.sort_unstable();

        let mut expected: Vec<String> = expected.iter().map(|value| (*value).to_owned()).collect();
        expected.sort_unstable();

        assert_eq!(response, expected);
    }

    #[test]
    fn example1() {
        assert_output("()())()", &["(())()", "()()()"]);
    }

    #[test]
    fn example2() {
        assert_output("(a)())()", &["(a())()", "(a)()()"]);
    }

    #[test]
    fn example3() {
        assert_output(")(", &[""]);
    }
}
