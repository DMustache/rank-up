struct Solution;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        logs.sort_unstable_by_key(|log| log[1]);

        let mut indexed_queries: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
        indexed_queries.sort_unstable_by_key(|&(_, time)| time);

        let mut result = vec![0; indexed_queries.len()];
        let mut request_counts = vec![0; n as usize + 1];
        let mut active_servers = 0;
        let mut left = 0;
        let mut right = 0;

        for (original_index, time) in indexed_queries {
            while right < logs.len() && logs[right][1] <= time {
                let server = logs[right][0] as usize;
                if request_counts[server] == 0 {
                    active_servers += 1;
                }
                request_counts[server] += 1;
                right += 1;
            }

            while left < right && logs[left][1] < time - x {
                let server = logs[left][0] as usize;
                request_counts[server] -= 1;
                if request_counts[server] == 0 {
                    active_servers -= 1;
                }
                left += 1;
            }

            result[original_index] = n - active_servers;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::count_servers(3, vec![vec![1, 3], vec![2, 6], vec![1, 5]], 5, vec![10, 11]),
            vec![1, 2]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::count_servers(
                3,
                vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]],
                2,
                vec![3, 4]
            ),
            vec![0, 1]
        );
    }

    #[test]
    fn inclusive_bounds_repeated_logs_and_unsorted_queries() {
        assert_eq!(
            Solution::count_servers(
                3,
                vec![vec![1, 5], vec![2, 4], vec![1, 3], vec![1, 5]],
                2,
                vec![6, 3, 5, 7, 4]
            ),
            vec![1, 2, 1, 2, 1]
        );
    }

    #[test]
    fn no_requests_in_window() {
        assert_eq!(
            Solution::count_servers(2, vec![vec![1, 1]], 0, vec![1, 2]),
            vec![1, 2]
        );
    }
}
