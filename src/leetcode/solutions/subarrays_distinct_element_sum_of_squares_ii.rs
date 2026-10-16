use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut last = HashMap::new();
        let mut positions = Fenwick::new(nums.len());
        let mut distinct = 0_i64;
        let mut position_sum = 0_i64;
        let mut ending_sum = 0_i64;
        let mut answer = 0_i64;

        for (i, value) in nums.into_iter().enumerate() {
            let previous = last.insert(value, i).map_or(-1, |p| p as i64);
            if previous >= 0 {
                let position = (previous + 1) as usize;
                positions.add(position, -1, -(previous + 1));
                distinct -= 1;
                position_sum -= previous + 1;
            }

            let (earlier_count, earlier_sum) = positions.prefix_sum((previous + 1) as usize);
            let later_count = distinct - earlier_count;
            let later_sum = position_sum - earlier_sum;

            ending_sum += 2 * (later_sum - later_count * (previous + 1)) + i as i64 - previous;
            answer = (answer + ending_sum) % MOD;

            positions.add(i + 1, 1, i as i64 + 1);
            distinct += 1;
            position_sum += i as i64 + 1;
        }

        answer as i32
    }
}

struct Fenwick {
    counts: Vec<i64>,
    sums: Vec<i64>,
}

impl Fenwick {
    fn new(len: usize) -> Self {
        Self {
            counts: vec![0; len + 1],
            sums: vec![0; len + 1],
        }
    }

    fn add(&mut self, mut index: usize, count: i64, sum: i64) {
        while index < self.counts.len() {
            self.counts[index] += count;
            self.sums[index] += sum;
            index += index & (!index + 1);
        }
    }

    fn prefix_sum(&self, mut index: usize) -> (i64, i64) {
        let mut count = 0;
        let mut sum = 0;
        while index > 0 {
            count += self.counts[index];
            sum += self.sums[index];
            index &= index - 1;
        }
        (count, sum)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
    }

    #[test]
    fn mixed_repetitions() {
        let nums = [1, 2, 1, 3, 2, 3];
        let expected: i32 = (0..nums.len())
            .flat_map(|start| (start..nums.len()).map(move |end| (start, end)))
            .map(|(start, end)| {
                let count = nums[start..=end]
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<_>>()
                    .len() as i32;
                count * count
            })
            .sum();
        assert_eq!(Solution::sum_counts(nums.to_vec()), expected);
    }
}
