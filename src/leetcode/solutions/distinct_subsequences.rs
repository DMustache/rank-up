
struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut dp = vec![0_i64; t.len() + 1];
        dp[0] = 1;

        for &s_char in s {
            for j in (1..=t.len()).rev() {
                if s_char == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[t.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}

