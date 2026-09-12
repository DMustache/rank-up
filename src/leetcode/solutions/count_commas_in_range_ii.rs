struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut total_commas: i64 = 0;
        let mut threshold: i64 = 1_000;

        while n >= threshold {
            total_commas += n - threshold + 1;
            match threshold.checked_mul(1_000) {
                Some(next_threshold) => threshold = next_threshold,
                None => break,
            }
        }
        total_commas
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_commas(1002), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_commas(998), 0);
    }
}
