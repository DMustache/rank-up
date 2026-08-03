struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut closest = nums[0] + nums[1] + nums.last().unwrap();

        for i in 0..nums.len() {
            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];

                if current_sum == target {
                    return current_sum;
                }

                if (current_sum - target).abs() < (closest - target).abs() {
                    closest = current_sum;
                }

                if current_sum > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        return closest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = vec![-1, 2, 1, -4];
        let p = 1;

        assert_eq!(2, Solution::three_sum_closest(case, p));
    }

    #[test]
    fn example2() {
        let case = vec![0, 0, 0];
        let p = 1;

        assert_eq!(0, Solution::three_sum_closest(case, p));
    }
}
