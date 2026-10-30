use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

#[derive(Clone, Debug)]
struct Node(usize, i64);

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let (n, k, labels) = (n as usize, k as usize, labels.as_bytes());

        let mut graph = vec![Vec::new(); n];
        for edge in edges {
            graph[edge[0] as usize].push(Node(edge[1] as usize, edge[2] as i64));
        }

        let stride = k + 1;
        let mut distance = vec![i64::MAX; n * stride];
        distance[1] = 0;
        let mut heap = BinaryHeap::from([Reverse((0_i64, 0_usize, 1_usize))]);

        while let Some(Reverse((cost, node, run))) = heap.pop() {
            if cost != distance[node * stride + run] {
                continue;
            }
            if node == n - 1 {
                return cost as i32;
            }

            for &Node(next, weight) in &graph[node] {
                let next_run = if labels[node] == labels[next] {
                    run + 1
                } else {
                    1
                };
                if next_run > k {
                    continue;
                }

                let next_cost = cost + weight;
                let index = next * stride + next_run;
                if next_cost < distance[index] {
                    distance[index] = next_cost;
                    heap.push(Reverse((next_cost, next, next_run)));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
        assert_eq!(
            Solution::shortest_path(3, edges.clone(), "aab".into(), 1),
            3
        );
        assert_eq!(Solution::shortest_path(3, edges, "aab".into(), 2), 2);
        assert_eq!(
            Solution::shortest_path(3, vec![vec![0, 1, 1], vec![1, 2, 1]], "aaa".into(), 2),
            -1
        );
    }

    #[test]
    fn one_node_and_unreachable_destination() {
        assert_eq!(Solution::shortest_path(1, vec![], "a".into(), 1), 0);
        assert_eq!(Solution::shortest_path(2, vec![], "ab".into(), 1), -1);
    }

    #[test]
    fn a_more_expensive_arrival_with_a_shorter_run_can_win() {
        let edges = vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![0, 3, 5],
            vec![3, 2, 1],
            vec![2, 4, 1],
        ];
        assert_eq!(Solution::shortest_path(5, edges, "aaaba".into(), 2), 7);
    }

    #[test]
    fn revisiting_a_node_can_reset_its_run() {
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 1, 1], vec![1, 3, 1]];
        assert_eq!(Solution::shortest_path(4, edges, "aaba".into(), 2), 4);
    }
}
