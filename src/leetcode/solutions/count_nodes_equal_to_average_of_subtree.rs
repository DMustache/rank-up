struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, matches) = Self::dfs(&root);

        matches
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        let Some(node) = node else {
            return (0, 0, 0);
        };

        let node = node.borrow();
        let (left_sum, left_count, left_matches) = Self::dfs(&node.left);
        let (right_sum, right_count, right_matches) = Self::dfs(&node.right);

        let sum = node.val + left_sum + right_sum;
        let count = 1 + left_count + right_count;
        let matches = left_matches + right_matches + if node.val == sum / count { 1 } else { 0 };

        (sum, count, matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_from_level_order(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(values: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(Some(value)) = values.get(index) else {
                return None;
            };

            Some(Rc::new(RefCell::new(TreeNode {
                val: *value,
                left: build(values, index * 2 + 1),
                right: build(values, index * 2 + 2),
            })))
        }

        build(values, 0)
    }

    #[test]
    fn example1() {
        let root =
            tree_from_level_order(&[Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)]);

        assert_eq!(Solution::average_of_subtree(root), 5);
    }

    #[test]
    fn example2() {
        let root = tree_from_level_order(&[Some(1)]);

        assert_eq!(Solution::average_of_subtree(root), 1);
    }
}
