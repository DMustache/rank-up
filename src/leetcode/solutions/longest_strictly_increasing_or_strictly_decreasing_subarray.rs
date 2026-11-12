struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut max_increassing, mut max_decreasing) = (0, 0);

        for index in 0..nums.len() {
            // остаток массива не длиннее уже найденного максимума — дальше искать нет смысла
            if nums.len() - index <= max_increassing.max(max_decreasing) as usize {
                break;
            }
            max_increassing = max_increassing.max(Self::increassing(&nums, index));
            max_decreasing = max_decreasing.max(Self::decreasing(&nums, index));
        }
        max_increassing.max(max_decreasing)
    }

    fn increassing(nums: &[i32], index: usize) -> i32 {
        if index + 1 < nums.len() && nums[index] < nums[index + 1] {
            return 1 + Self::increassing(nums, index + 1);
        }
        1
    }

    fn decreasing(nums: &[i32], index: usize) -> i32 {
        if index + 1 < nums.len() && nums[index] > nums[index + 1] {
            return 1 + Self::decreasing(nums, index + 1);
        }
        1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![1, 4, 3, 3, 2];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 2);
    }

    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![3, 3, 3, 3];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 1);
    }

    #[test]
    fn example3() {
        let nums: Vec<i32> = vec![3, 2, 1];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 3);
    }
}
