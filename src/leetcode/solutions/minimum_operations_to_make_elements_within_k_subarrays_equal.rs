struct Solution;

struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, index: usize, delta: i64) {
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn prefix(&self, end: usize) -> i64 {
        let mut result = 0;
        let mut i = end;
        while i > 0 {
            result += self.tree[i];
            i &= i - 1;
        }
        result
    }

    fn kth(&self, rank: i64) -> usize {
        let mut index = 0;
        let mut bit = 1;
        while bit < self.tree.len() {
            bit <<= 1;
        }
        let mut remaining = rank;
        while bit > 0 {
            let next = index + bit;
            if next < self.tree.len() && self.tree[next] < remaining {
                remaining -= self.tree[next];
                index = next;
            }
            bit >>= 1;
        }
        index
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;

        let mut values = nums.clone();
        values.sort_unstable();
        values.dedup();
        let indices: Vec<usize> = nums
            .iter()
            .map(|value| values.binary_search(value).unwrap())
            .collect();

        let mut counts = Fenwick::new(values.len());
        let mut sums = Fenwick::new(values.len());
        let mut total_sum = 0_i64;
        for i in 0..x {
            counts.add(indices[i], 1);
            sums.add(indices[i], nums[i] as i64);
            total_sum += nums[i] as i64;
        }

        let mut cost = vec![0_i64; n - x + 1];
        for start in 0..cost.len() {
            let median_index = counts.kth((x as i64 + 1) / 2);
            let median = values[median_index] as i64;
            let left_count = counts.prefix(median_index + 1);
            let left_sum = sums.prefix(median_index + 1);
            cost[start] = median * left_count - left_sum + total_sum
                - left_sum
                - median * (x as i64 - left_count);

            if start + x < n {
                counts.add(indices[start], -1);
                sums.add(indices[start], -(nums[start] as i64));
                total_sum -= nums[start] as i64;
                counts.add(indices[start + x], 1);
                sums.add(indices[start + x], nums[start + x] as i64);
                total_sum += nums[start + x] as i64;
            }
        }

        let inf = i64::MAX / 4;
        let mut previous = vec![0_i64; n + 1];
        for _ in 0..k {
            let mut current = vec![inf; n + 1];
            for end in x..=n {
                current[end] = current[end - 1].min(previous[end - x] + cost[end - x]);
            }
            previous = current;
        }
        previous[n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
            8
        );
        assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }

    #[test]
    fn repeated_values_and_extremes() {
        assert_eq!(Solution::min_operations(vec![7, 7, 1, 2, 7, 7], 2, 2), 0);
        assert_eq!(
            Solution::min_operations(vec![-1_000_000, 1_000_000], 2, 1),
            2_000_000
        );
    }
}
