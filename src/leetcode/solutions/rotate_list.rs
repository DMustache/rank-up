struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut node = head.as_ref();
        while let Some(current) = node {
            len += 1;
            node = current.next.as_ref();
        }

        if len == 0 {
            return head;
        }

        let shift = k as usize % len;
        if shift == 0 {
            return head;
        }

        let mut head = head;
        let mut split = &mut head;
        for _ in 1..(len - shift) {
            split = &mut split.as_mut().unwrap().next;
        }

        let mut new_head = split.as_mut().unwrap().next.take();
        let mut tail = new_head.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = head;

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;

        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = head;
            head = Some(node);
        }

        head
    }

    #[test]
    fn example1() {
        assert_eq!(
            list_from(&[4, 5, 1, 2, 3]),
            Solution::rotate_right(list_from(&[1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            list_from(&[2, 0, 1]),
            Solution::rotate_right(list_from(&[0, 1, 2]), 4)
        );
    }

    #[test]
    fn empty_list() {
        assert_eq!(None, Solution::rotate_right(None, 3));
    }

    #[test]
    fn zero_rotations() {
        assert_eq!(
            list_from(&[1, 2, 3]),
            Solution::rotate_right(list_from(&[1, 2, 3]), 0)
        );
    }

    #[test]
    fn full_rotation() {
        assert_eq!(
            list_from(&[1, 2, 3]),
            Solution::rotate_right(list_from(&[1, 2, 3]), 3)
        );
    }
}
