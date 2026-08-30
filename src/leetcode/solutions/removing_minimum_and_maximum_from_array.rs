struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
       let mut minimum_index = 0;
        let mut minimum = &nums[minimum_index];
        let mut maximum_index = 0;
        let mut maximum = &nums[maximum_index];

        for (index, num) in nums.iter().enumerate() {
            if num < minimum {
                minimum = num;
                minimum_index = index;
            }
            if num > maximum {
                maximum = num;
                maximum_index = index;
            }
        }

        let lenght = nums.len();
        let left = minimum_index.min(maximum_index);
        let right = minimum_index.max(maximum_index);

        let front = right + 1;
        let back = lenght - left;
        let both = left + 1 + lenght - right;

        front.min(back).min(both) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::minimum_deletions(vec![101]), 1);
    }
}
