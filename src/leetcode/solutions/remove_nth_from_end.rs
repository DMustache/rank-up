struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut lenght: usize = 0;

        let mut current = head.as_ref();
        while current.is_some() {
            lenght += 1;
            current = current.and_then(|value| value.next.as_ref());
        }

        let mut result: Option<Box<ListNode>> = head;
        let mut pointer = &mut result;
        for _ in 0..lenght - n as usize {
            pointer = &mut pointer.as_mut().unwrap().next;
        }

        let next = pointer.as_mut().unwrap().next.take();
        *pointer = next;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;

        for value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(*value));
            node.next = head;
            head = Some(node);
        }

        head
    }

    #[test]
    fn example1() {
        let head = list_from(&[1, 2, 3, 4, 5]);
        let n = 2;

        assert_eq!(
            list_from(&[1, 2, 3, 5]),
            Solution::remove_nth_from_end(head, n)
        );
    }

    #[test]
    fn example2() {
        let head = list_from(&[1]);
        let n = 1;

        assert_eq!(list_from(&[]), Solution::remove_nth_from_end(head, n));
    }

    #[test]
    fn example3() {
        let head = list_from(&[1, 2]);
        let n = 1;

        assert_eq!(list_from(&[1]), Solution::remove_nth_from_end(head, n));
    }
}
