use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut free = BinaryHeap::new();
        for (index, weight) in servers.into_iter().enumerate() {
            free.push(Reverse((weight, index)));
        }

        let mut busy = BinaryHeap::new();
        let mut result = Vec::with_capacity(tasks.len());
        let mut time = 0;

        for (arrival, duration) in tasks.into_iter().enumerate() {
            time = time.max(arrival as i64);
            if free.is_empty() {
                let &Reverse((finish, _, _)) = busy.peek().unwrap();
                time = time.max(finish);
            }

            while let Some(&Reverse((finish, weight, index))) = busy.peek() {
                if finish > time {
                    break;
                }
                busy.pop();
                free.push(Reverse((weight, index)));
            }

            let Reverse((weight, index)) = free.pop().unwrap();
            result.push(index as i32);
            busy.push(Reverse((time + duration as i64, weight, index)));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn first_example() {
        assert_eq!(
            Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
            vec![2, 2, 0, 2, 1, 2]
        );
    }

    #[test]
    fn second_example() {
        assert_eq!(
            Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
            vec![1, 4, 1, 4, 1, 3, 2]
        );
    }

    #[test]
    fn queued_tasks_and_simultaneous_release() {
        assert_eq!(
            Solution::assign_tasks(vec![2, 1], vec![5, 4, 1, 1]),
            vec![1, 0, 1, 0]
        );
    }

    #[test]
    fn finish_time_can_exceed_i32() {
        assert_eq!(
            Solution::assign_tasks(vec![1], vec![2_000_000_000, 2_000_000_000, 1]),
            vec![0, 0, 0]
        );
    }
}
