use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn weighted_sum(parent: Vec<i32>, nums: Vec<i32>) -> i64 {
        if parent.is_empty() {
            return 0;
        }

        let mut children = vec![Vec::new(); parent.len()];
        for index in 1..parent.len() {
            children[parent[index] as usize].push(index);
        }

        let mut depths = vec![0; parent.len()];
        depths[0] = 1;
        let mut height = 1;
        let mut queue = VecDeque::from([0]);

        while let Some(node) = queue.pop_front() {
            for &child in &children[node] {
                depths[child] = depths[node] + 1;
                height = height.max(depths[child]);
                queue.push_back(child);
            }
        }

        let mut result = 0;
        for (index, &value) in nums.iter().enumerate() {
            let weight = height - depths[index] + 1;
            result += value as i64 * weight;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::weighted_sum(vec![-1, 0, 0, 0, 2, 2], vec![5, 2, 3, 1, 4, 6]),
            37
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::weighted_sum(vec![-1, 0, 1, 2], vec![1, 2, 3, 4]),
            20
        );
    }

    #[test]
    fn parent_can_appear_after_child() {
        assert_eq!(
            Solution::weighted_sum(vec![-1, 3, 0, 0], vec![1, 2, 3, 4]),
            19
        );
    }
}
