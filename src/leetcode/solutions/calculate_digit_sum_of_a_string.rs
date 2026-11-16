struct Solution;

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;

        while s.len() > k {
            let mut next = String::new();
            for group in s.as_bytes().chunks(k) {
                let sum: u32 = group.iter().map(|digit| (digit - b'0') as u32).sum();
                next.push_str(&sum.to_string());
            }
            s = next;
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn digit_sum_examples() {
        assert_eq!(Solution::digit_sum("11111222223".into(), 3), "135");
        assert_eq!(Solution::digit_sum("00000000".into(), 3), "000");
    }

    #[test]
    fn digit_sum_keeps_short_strings() {
        assert_eq!(Solution::digit_sum("123".into(), 3), "123");
    }
}
