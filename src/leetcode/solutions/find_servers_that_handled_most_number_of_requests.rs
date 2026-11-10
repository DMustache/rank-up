use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let server_count = k as usize;
        let mut busy = BinaryHeap::<Reverse<(i64, usize)>>::new();
        let mut available = (0..server_count).collect::<BTreeSet<_>>();
        let mut request_count = vec![0; server_count];

        for (index, (&arrival_time, &duration)) in arrival.iter().zip(&load).enumerate() {
            while let Some(&Reverse((free_time, server))) = busy.peek() {
                if free_time > i64::from(arrival_time) {
                    break;
                }
                busy.pop();
                available.insert(server);
            }

            let preferred = index % server_count;
            let server = available
                .range(preferred..)
                .next()
                .or_else(|| available.iter().next())
                .copied();

            if let Some(server) = server {
                available.remove(&server);
                request_count[server] += 1;
                busy.push(Reverse((
                    i64::from(arrival_time) + i64::from(duration),
                    server,
                )));
            }
        }

        let most_requests = request_count.iter().copied().max().unwrap_or(0);
        request_count
            .into_iter()
            .enumerate()
            .filter_map(|(server, count)| (count == most_requests).then_some(server as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
            vec![1]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
            vec![0]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
            vec![0, 1, 2]
        );
    }
}
