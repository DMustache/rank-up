struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut result = 0;
        let reverse_value = |ch: u8| (b'z' - ch + 1) as i32;

        for (index, ch) in s.bytes().enumerate() {
            result += reverse_value(ch) * (index as i32 + 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_degree("zaza".to_string()), 160);
    }
}
