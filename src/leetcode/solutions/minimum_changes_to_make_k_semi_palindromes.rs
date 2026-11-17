struct Solution;

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        let chars = s.as_bytes();
        let mut divisors = vec![Vec::<usize>::new(); n + 1];

        for d in 1..n {
            let mut length = 2 * d;
            while length <= n {
                divisors[length].push(d);
                length += d;
            }
        }

        let mut cost = vec![vec![n as i32; n + 1]; n + 1];

        for start in 0..n {
            for end in start + 2..=n {
                for &d in &divisors[end - start] {
                    let mut changes = 0;

                    for offset in 0..d {
                        let mut left = start + offset;
                        let mut right = end - d + offset;

                        while left < right {
                            if chars[left] != chars[right] {
                                changes += 1;
                            }
                            left += d;
                            right -= d;
                        }
                    }

                    cost[start][end] = cost[start][end].min(changes);
                }
            }
        }

        const INF: i32 = 1_000_000_000;
        let mut dp = vec![vec![INF; n + 1]; k + 1];
        dp[0][0] = 0;

        for p in 1..=k {
            for end in 2 * p..=n {
                for start in 2 * (p - 1)..end - 1 {
                    dp[p][end] = dp[p][end].min(dp[p - 1][start] + cost[start][end]);
                }
            }
        }

        dp[k][n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;

        assert_eq!(Solution::minimum_changes(s, k), 1);
    }

    #[test]
    fn example2() {
        let s: String = "abcdef".to_string();
        let k: i32 = 2;

        assert_eq!(Solution::minimum_changes(s, k), 2);
    }

    #[test]
    fn example3() {
        let s: String = "aabbaa".to_string();
        let k: i32 = 3;

        assert_eq!(Solution::minimum_changes(s, k), 0);
    }

    #[test]
    fn nontrivial_divisor() {
        assert_eq!(Solution::minimum_changes("abcdabcd".to_string(), 1), 0);
    }
}
