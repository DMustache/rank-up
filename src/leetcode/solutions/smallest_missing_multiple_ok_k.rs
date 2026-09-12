use std::collections::HashSet;

struct Solution;

struct MultipleOfK {
    current: i32,
    k: i32,
}

impl MultipleOfK {
    pub fn new(k: i32) -> Self {
        Self { current: 0, k }
    }
}

impl Iterator for MultipleOfK {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.k;
        Some(self.current)
    }
}

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut numbers_set: HashSet<i32> = nums.into_iter().collect();

        MultipleOfK::new(k)
            .find(|multiple| !numbers_set.remove(multiple))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![8, 2, 3, 4, 6];

        assert_eq!(10, Solution::missing_multiple(nums, 2));
    }

    #[test]
    fn example2() {
        let nums = vec![1, 4, 7, 10, 15];

        assert_eq!(5, Solution::missing_multiple(nums, 5));
    }
}
