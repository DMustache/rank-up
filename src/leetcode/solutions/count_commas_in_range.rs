struct Solution;

impl Solution {
    // sum[k=1,inf]( max(0, n - 10^3k +1) )
    pub fn count_commas(n: i32) -> i32 {
        let mut total_commas: i32 = 0;
        let mut threshold: i32 = 1_000;

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
