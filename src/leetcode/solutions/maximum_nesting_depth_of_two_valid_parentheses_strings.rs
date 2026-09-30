struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depth = 0;
        let mut groups = Vec::with_capacity(seq.len());

        for ch in seq.bytes() {
            if ch == b'(' {
                groups.push(depth % 2);
                depth += 1;
            } else {
                depth -= 1;
                groups.push(depth % 2);
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn max_group_depth(seq: &str, groups: &[i32]) -> i32 {
        assert_eq!(seq.len(), groups.len());
        let mut depths = [0, 0];
        let mut maximum = 0;

        for (ch, &group) in seq.bytes().zip(groups) {
            assert!((0..=1).contains(&group));
            let depth = &mut depths[group as usize];
            if ch == b'(' {
                *depth += 1;
                maximum = maximum.max(*depth);
            } else {
                *depth -= 1;
                assert!(*depth >= 0);
            }
        }

        assert_eq!(depths, [0, 0]);
        maximum
    }

    #[test]
    fn example_1() {
        let seq = "(()())";
        let groups = Solution::max_depth_after_split(seq.to_string());
        assert_eq!(groups, vec![0, 1, 1, 1, 1, 0]);
        assert_eq!(max_group_depth(seq, &groups), 1);
    }

    #[test]
    fn example_2() {
        let seq = "()(())()";
        let groups = Solution::max_depth_after_split(seq.to_string());
        assert_eq!(max_group_depth(seq, &groups), 1);
        assert_eq!(max_group_depth(seq, &[0, 0, 0, 1, 1, 0, 1, 1]), 1);
    }

    #[test]
    fn deeply_nested_parentheses() {
        let seq = "(((())))";
        let groups = Solution::max_depth_after_split(seq.to_string());
        assert_eq!(max_group_depth(seq, &groups), 2);
    }
}
