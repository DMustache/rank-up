struct Solution;

#[derive(Clone)]
struct Node {
    product: usize,
    prefixes: Vec<i32>,
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let identity = || Node {
            product: 1 % k,
            prefixes: vec![0; k],
        };

        let merge = |left: &Node, right: &Node| {
            let mut prefixes = left.prefixes.clone();

            for (remainder, &count) in right.prefixes.iter().enumerate() {
                let shifted = (left.product * remainder) % k;
                prefixes[shifted] += count;
            }

            Node {
                product: (left.product * right.product) % k,
                prefixes,
            }
        };

        let mut size = 1;
        while size < n {
            size *= 2;
        }

        let mut tree = vec![identity(); size * 2];
        for (index, &value) in nums.iter().enumerate() {
            let remainder = value as usize % k;
            tree[size + index] = Node {
                product: remainder,
                prefixes: {
                    let mut prefixes = vec![0; k];
                    prefixes[remainder] = 1;
                    prefixes
                },
            };
        }

        for index in (1..size).rev() {
            tree[index] = merge(&tree[index * 2], &tree[index * 2 + 1]);
        }

        let mut result = Vec::with_capacity(queries.len());

        for query in queries {
            let index = query[0] as usize;
            let value = query[1] as usize % k;
            let start = query[2] as usize;
            let x = query[3] as usize;

            let leaf = size + index;
            tree[leaf] = Node {
                product: value,
                prefixes: {
                    let mut prefixes = vec![0; k];
                    prefixes[value] = 1;
                    prefixes
                },
            };

            let mut current = leaf / 2;
            while current > 0 {
                tree[current] = merge(&tree[current * 2], &tree[current * 2 + 1]);
                current /= 2;
            }

            let mut left_result = identity();
            let mut right_result = identity();
            let mut left = size + start;
            let mut right = size + n;

            while left < right {
                if left % 2 == 1 {
                    left_result = merge(&left_result, &tree[left]);
                    left += 1;
                }

                if right % 2 == 1 {
                    right -= 1;
                    right_result = merge(&tree[right], &right_result);
                }

                left /= 2;
                right /= 2;
            }

            let answer = merge(&left_result, &right_result);
            result.push(answer.prefixes[x]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::result_array(
                vec![1, 2, 3, 4, 5],
                3,
                vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]],
            ),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::result_array(
                vec![1, 2, 4, 8, 16, 32],
                4,
                vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]],
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::result_array(vec![1, 1, 2, 1, 1], 2, vec![vec![2, 1, 0, 1]]),
            vec![5]
        );
    }
}
