struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = Vec::new();
        let mut stars = Vec::new();

        for (index, ch) in s.chars().enumerate() {
            if !Self::check_any_bracket(&mut open, &mut stars, index, ch) {
                return false;
            }
        }

        while let Some(open_index) = open.pop() {
            match stars.pop() {
                Some(star_index) if star_index > open_index => {}
                _ => return false,
            }
        }

        true
    }

    fn check_any_bracket(
        open: &mut Vec<usize>,
        stars: &mut Vec<usize>,
        index: usize,
        ch: char,
    ) -> bool {
        match ch {
            '(' => {
                open.push(index);
                true
            }
            ')' => open.pop().is_some() || stars.pop().is_some(),
            '*' => {
                stars.push(index);
                true
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::check_valid_string("()".to_string()), true)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::check_valid_string("(*)".to_string()), true)
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::check_valid_string("(*))".to_string()), true)
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::check_valid_string("(".to_string()), false)
    }

    #[test]
    fn revealed() {
        assert!(!Solution::check_valid_string("((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((".to_owned()))
    }

    #[test]
    fn wildcard_positions() {
        for s in ["", "*", "(*", "*)", "(()*", "*()"] {
            assert!(Solution::check_valid_string(s.to_owned()), "{s}");
        }
        for s in ["*(", ")*", "(*(", "())*"] {
            assert!(!Solution::check_valid_string(s.to_owned()), "{s}");
        }
    }
}
