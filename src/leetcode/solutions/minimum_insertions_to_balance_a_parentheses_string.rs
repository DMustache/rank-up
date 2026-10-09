struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0;
        let mut needed_closes = 0;

        for byte in s.bytes() {
            if byte == b'(' {
                if needed_closes % 2 == 1 {
                    insertions += 1;
                    needed_closes -= 1;
                }
                needed_closes += 2;
            } else {
                needed_closes -= 1;
                if needed_closes < 0 {
                    insertions += 1;
                    needed_closes = 1;
                }
            }
        }

        insertions + needed_closes
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_insertions("())".to_string()), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    }
}
