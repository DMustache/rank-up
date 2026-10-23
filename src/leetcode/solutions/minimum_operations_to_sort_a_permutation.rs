struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let zero = nums.iter().position(|&value| value == 0).unwrap();
        let mut targets = [false; 2];

        for (reversed, target) in targets.iter_mut().enumerate() {
            let mut index = zero;
            *target = (0..nums.len()).all(|value| {
                let matches = nums[index] == value as i32;
                index = if reversed == 0 {
                    (index + 1) % nums.len()
                } else {
                    (index + nums.len() - 1) % nums.len()
                };
                matches
            });
        }

        if !targets[0] && !targets[1] {
            return -1;
        }

        let mut distance = vec![-1; 2 * nums.len()];
        let mut queue = VecDeque::from([0usize]);
        distance[0] = 0;

        while let Some(state) = queue.pop_front() {
            let first = state / 2;
            let reversed = state % 2;
            let steps = distance[state];
            if first == zero && targets[reversed] {
                return steps;
            }

            let rotated_first = if reversed == 0 {
                (first + 1) % nums.len()
            } else {
                (first + nums.len() - 1) % nums.len()
            };
            let reversed_first = if reversed == 0 {
                (first + nums.len() - 1) % nums.len()
            } else {
                (first + 1) % nums.len()
            };
            let next_states = [
                rotated_first * 2 + reversed,
                reversed_first * 2 + 1 - reversed,
            ];
            for next in next_states {
                if distance[next] == -1 {
                    distance[next] = steps + 1;
                    queue.push_back(next);
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::solutions::minimum_operations_to_sort_a_permutation::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_operations(vec![0, 2, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_operations(vec![1, 0, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_operations(vec![2, 0, 1, 3]), -1);
    }

    #[test]
    fn other_reachable_arrangements() {
        assert_eq!(Solution::min_operations(vec![0]), 0);
        assert_eq!(Solution::min_operations(vec![0, 1, 2, 3]), 0);
        assert_eq!(Solution::min_operations(vec![3, 0, 1, 2]), 1);
        assert_eq!(Solution::min_operations(vec![3, 2, 1, 0]), 1);
        assert_eq!(Solution::min_operations(vec![2, 1, 0, 3]), 2);
    }
}
