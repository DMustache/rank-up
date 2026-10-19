struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = 0;
        let mut shift = 0;

        for num in nums.iter() {
            if num < &target {
                shift += 1;
            }

            if num == &target {
                result += 1;
            }
        }

        let mut indexes = Vec::new();

        for i in shift..shift + result {
            indexes.push(i);
        }

        indexes
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 2;
        assert_eq!(Solution::target_indices(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 3;
        assert_eq!(Solution::target_indices(nums, target), vec![3]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 5;
        assert_eq!(Solution::target_indices(nums, target), vec![4]);
    }

    #[test]
    fn test_target_not_found() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 4;
        assert_eq!(Solution::target_indices(nums, target), Vec::<i32>::new());
    }

    #[test]
    fn test_multiple_occurrences() {
        let nums = vec![4, 4, 4, 4, 4];
        let target = 4;
        assert_eq!(Solution::target_indices(nums, target), vec![0, 1, 2, 3, 4]);
    }
}
