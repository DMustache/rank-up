struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intervals: Vec<(i32, i32, i32, i32)> = intervals
            .into_iter()
            .enumerate()
            .map(|(index, interval)| (interval[0], interval[1], interval[2], index as i32))
            .collect();
        intervals.sort_unstable_by_key(|interval| interval.0);

        let starts: Vec<i32> = intervals.iter().map(|interval| interval.0).collect();
        let next: Vec<usize> = intervals
            .iter()
            .map(|interval| starts.partition_point(|&start| start <= interval.1))
            .collect();

        fn better(left: (i64, Vec<i32>), right: (i64, Vec<i32>)) -> (i64, Vec<i32>) {
            if left.0 != right.0 {
                return if left.0 > right.0 { left } else { right };
            }

            if left.1 <= right.1 { left } else { right }
        }

        let mut dp = vec![vec![(0_i64, Vec::new()); 5]; intervals.len() + 1];
        for position in (0..intervals.len()).rev() {
            for remaining in 1..=4 {
                let skip = dp[position + 1][remaining].clone();
                let (score, mut indices) = dp[next[position]][remaining - 1].clone();
                indices.push(intervals[position].3);
                indices.sort_unstable();

                let take = (score + intervals[position].2 as i64, indices);
                dp[position][remaining] = better(skip, take);
            }
        }

        dp[0][4].1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1],
        ];

        assert_eq!(Solution::maximum_weight(intervals), vec![2, 3]);
    }

    #[test]
    fn example_2() {
        let intervals = vec![
            vec![5, 8, 1],
            vec![6, 7, 7],
            vec![4, 7, 3],
            vec![9, 10, 6],
            vec![7, 8, 2],
            vec![11, 14, 3],
            vec![3, 5, 5],
        ];

        assert_eq!(Solution::maximum_weight(intervals), vec![1, 3, 5, 6]);
    }
}
