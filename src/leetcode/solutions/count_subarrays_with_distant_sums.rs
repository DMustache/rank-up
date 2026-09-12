struct Solution;

impl Solution {
    pub fn distant_subarrays(nums: Vec<i32>, goal: i32, k: i32) -> i64 {
        let mut prefix_sums = vec![0];
        for number in nums {
            prefix_sums.push(prefix_sums.last().unwrap() + number as i64);
        }

        let mut coordinates = prefix_sums.clone();
        coordinates.sort_unstable();
        coordinates.dedup();

        let mut fenwick = Fenwick::new(coordinates.len());
        let mut not_distant = 0;
        let goal = goal as i64;
        let k = k as i64;

        for &prefix_sum in &prefix_sums {
            let lower = prefix_sum - goal - k;
            let upper = prefix_sum - goal + k;
            let less_than_upper = coordinates.partition_point(|&value| value < upper);
            let less_or_equal_lower = coordinates.partition_point(|&value| value <= lower);

            if lower < upper {
                not_distant += fenwick.sum(less_than_upper) - fenwick.sum(less_or_equal_lower);
            }
            let coordinate = coordinates.binary_search(&prefix_sum).unwrap();
            fenwick.add(coordinate + 1, 1);
        }

        let length = prefix_sums.len() as i64 - 1;
        length * (length + 1) / 2 - not_distant
    }
}

struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, mut index: usize, value: i64) {
        while index < self.tree.len() {
            self.tree[index] += value;
            index += index & (!index + 1);
        }
    }

    fn sum(&self, mut count: usize) -> i64 {
        let mut result = 0;
        while count > 0 {
            result += self.tree[count];
            count &= count - 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::distant_subarrays(vec![1, 2, 1], 4, 1), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::distant_subarrays(vec![2, -1, 3], 2, 2), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::distant_subarrays(vec![-3, 1, 2], 0, 3), 2);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::distant_subarrays(vec![16, 26, 41, 20, -25, 18], -7, 0),
            21
        );
    }
}
