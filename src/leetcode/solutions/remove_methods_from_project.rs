struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n as usize];

        for invocation in &invocations {
            graph[invocation[0] as usize].push(invocation[1] as usize)
        }

        let mut suspicious = vec![false; n as usize];
        let mut stack: Vec<usize> = vec![k as usize];
        suspicious[k as usize] = true;

        while let Some(current) = stack.pop() {
            for &next in &graph[current] {
                if !suspicious[next] {
                    suspicious[next] = true;
                    stack.push(next);
                }
            }
        }

        for invocation in &invocations {
            if !suspicious[invocation[0] as usize] && suspicious[invocation[1] as usize] {
                return (0..n).collect();
            }
        }

        (0..n)
            .filter(|&method| !suspicious[method as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let n = 4;
        let k = 1;
        let invocations = vec![vec![1, 2], vec![0, 1], vec![3, 2]];

        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::remaining_methods(n, k, invocations)
        );
    }

    #[test]
    fn example2() {
        let n = 5;
        let k = 0;
        let invocations = vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]];

        assert_eq!(vec![3, 4], Solution::remaining_methods(n, k, invocations));
    }

    #[test]
    fn example3() {
        let n = 3;
        let k = 2;
        let invocations = vec![vec![1, 2], vec![0, 1], vec![2, 0]];

        assert_eq!(
            Vec::<i32>::new(),
            Solution::remaining_methods(n, k, invocations)
        );
    }
}
