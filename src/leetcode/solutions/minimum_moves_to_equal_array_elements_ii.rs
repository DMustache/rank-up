struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let median = nums[nums.len() / 2];
        nums.iter().map(|&x| (x - median).abs()).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    }
}
