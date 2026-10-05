struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut level = 0;
        let mut score = 0;
        let mut previous = ' ';

        for ch in s.chars() {
            match ch {
                '(' => level += 1,
                ')' => {
                    level -= 1;
                    if previous == '(' {
                        score += 1 << level;
                    }
                }
                _ => unreachable!(),
            }
            previous = ch;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2)
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2)
    }

    #[test]
    fn nested_and_adjacent() {
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6)
    }
}
