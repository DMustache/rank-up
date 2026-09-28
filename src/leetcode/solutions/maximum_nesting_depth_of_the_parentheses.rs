struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut current_depth = 0;

        for ch in s.bytes() {
            match ch {
                b'(' => current_depth += 1,
                b')' => {
                    depth = depth.max(current_depth);
                    current_depth -= 1;
                }
                _ => (),
            }
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_depth("()(())((()()))".to_string()), 3);
    }
}
