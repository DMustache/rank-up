struct Solution;

#[derive(Clone, Copy, Default)]
struct Node {
    left_char: u8,
    right_char: u8,
    prefix: usize,
    suffix: usize,
    longest: usize,
    length: usize,
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        fn merge(left: Node, right: Node) -> Node {
            let prefix = if left.prefix == left.length && left.right_char == right.left_char {
                left.length + right.prefix
            } else {
                left.prefix
            };
            let suffix = if right.suffix == right.length && left.right_char == right.left_char {
                right.length + left.suffix
            } else {
                right.suffix
            };
            let joined = if left.right_char == right.left_char {
                left.suffix + right.prefix
            } else {
                0
            };

            Node {
                left_char: left.left_char,
                right_char: right.right_char,
                prefix,
                suffix,
                longest: left.longest.max(right.longest).max(joined),
                length: left.length + right.length,
            }
        }

        fn build(tree: &mut [Node], values: &[u8], tree_index: usize, left: usize, right: usize) {
            if left == right {
                tree[tree_index] = Node {
                    left_char: values[left],
                    right_char: values[left],
                    prefix: 1,
                    suffix: 1,
                    longest: 1,
                    length: 1,
                };
                return;
            }

            let middle = (left + right) / 2;
            build(tree, values, tree_index * 2, left, middle);
            build(tree, values, tree_index * 2 + 1, middle + 1, right);
            tree[tree_index] = merge(tree[tree_index * 2], tree[tree_index * 2 + 1]);
        }

        fn update(
            tree: &mut [Node],
            value: u8,
            tree_index: usize,
            left: usize,
            right: usize,
            index: usize,
        ) {
            if left == right {
                tree[tree_index].left_char = value;
                tree[tree_index].right_char = value;
                return;
            }

            let middle = (left + right) / 2;
            if index <= middle {
                update(tree, value, tree_index * 2, left, middle, index);
            } else {
                update(tree, value, tree_index * 2 + 1, middle + 1, right, index);
            }
            tree[tree_index] = merge(tree[tree_index * 2], tree[tree_index * 2 + 1]);
        }

        let values = s.into_bytes();
        let mut tree = vec![Node::default(); values.len() * 4];
        build(&mut tree, &values, 1, 0, values.len() - 1);

        query_characters
            .into_bytes()
            .into_iter()
            .zip(query_indices)
            .map(|(character, index)| {
                update(&mut tree, character, 1, 0, values.len() - 1, index as usize);
                tree[1].longest as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3],),
            vec![3, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1]),
            vec![2, 3]
        );
    }

    #[test]
    fn updates_single_character_string() {
        assert_eq!(
            Solution::longest_repeating("a".to_string(), "b".to_string(), vec![0]),
            vec![1]
        );
    }

    #[test]
    fn updates_can_split_and_restore_runs() {
        assert_eq!(
            Solution::longest_repeating("aaaa".to_string(), "bbaa".to_string(), vec![1, 2, 1, 2],),
            vec![2, 2, 3, 4]
        );
    }
}
