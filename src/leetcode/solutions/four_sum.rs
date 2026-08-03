struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        k_sum(&nums, target as i64, 4, 0)
    }
}

fn k_sum(nums: &[i32], target: i64, size: usize, start: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if size == 2 {
        return two_sum(nums, target, start);
    }

    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }

        if nums.len() - i < size {
            break;
        }

        for mut sum in k_sum(nums, target - nums[i] as i64, size - 1, i + 1) {
            let mut combination = vec![nums[i]];
            combination.append(&mut sum);
            result.push(combination);
        }
    }

    result
}

fn two_sum(nums: &[i32], target: i64, start: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut left = start;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] as i64 + nums[right] as i64;

        if sum == target {
            result.push(vec![nums[left], nums[right]]);
            left += 1;
            right -= 1;

            while left < right && nums[left] == nums[left - 1] {
                left += 1;
            }

            while left < right && nums[right] == nums[right + 1] {
                right -= 1;
            }
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = vec![1, 0, -1, 0, -2, 2];
        let target = 0;

        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(case, target)
        );
    }

    #[test]
    fn example2() {
        let case = vec![2, 2, 2, 2, 2];
        let target = 8;

        assert_eq!(vec![vec![2, 2, 2, 2]], Solution::four_sum(case, target));
    }
}
