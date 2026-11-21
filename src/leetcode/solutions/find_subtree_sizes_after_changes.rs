use std::ptr::{self, null_mut};

struct Solution;

struct Node {
    id: usize,
    char_id: usize,

    origin: *mut Parent,
    new: *mut Parent,

    subtree_size: usize,
}

struct Parent {
    pub parent: *mut Node,
    pub first_child: *mut Node,
    pub sibling: *mut Node,
}

impl Parent {
    fn new() -> *mut Self {
        Box::into_raw(Box::new(Self {
            parent: null_mut(),
            first_child: null_mut(),
            sibling: null_mut(),
        }))
    }
}

impl Node {
    fn new(id: usize, char_id: u8) -> *mut Node {
        Box::into_raw(Box::new(Self {
            id,
            char_id: (char_id - b'a') as usize,
            origin: Parent::new(),
            new: Parent::new(),
            subtree_size: 0,
        }))
    }

    unsafe fn add_orig_child(parent: *mut Node, child: *mut Node) {
        unsafe {
            let parent_links = (*parent).origin;
            let child_links = (*child).origin;
            (*child_links).parent = parent;
            (*child_links).sibling = (*parent_links).first_child;
            (*parent_links).first_child = child;
        }
    }

    unsafe fn add_new_child(parent: *mut Node, child: *mut Node) {
        unsafe {
            let parent_links = (*parent).new;
            let child_links = (*child).new;
            (*child_links).parent = parent;
            (*child_links).sibling = (*parent_links).first_child;
            (*parent_links).first_child = child;
        }
    }
}

struct Graph {
    nodes: *mut *mut Node,
    len: usize,
}

impl Graph {
    fn from_input(parent: &[i32], s: &str) -> Self {
        let n = parent.len();
        let bytes = s.as_bytes();
        let buffer: Vec<*mut Node> = (0..n).map(|i| Node::new(i, bytes[i])).collect();

        let boxed_slice = buffer.into_boxed_slice();
        let len = boxed_slice.len();
        let nodes = Box::into_raw(boxed_slice) as *mut *mut Node;

        unsafe {
            for x in 1..len {
                let pointer_id = parent[x];
                let p_pointer = *nodes.add(pointer_id as usize);
                let x_pointer = *nodes.add(x);
                Node::add_orig_child(p_pointer, x_pointer);
            }
        }

        Self { nodes, len }
    }

    unsafe fn dfs_relink(u: *mut Node, last_seen: &mut [*mut Node; 26]) {
        unsafe {
            let c = (*u).char_id;
            let prev = last_seen[c];

            if (*u).id != 0 {
                let target_parent = if !prev.is_null() {
                    prev
                } else {
                    (*(*u).origin).parent
                };
                Node::add_new_child(target_parent, u);
            }

            last_seen[c] = u;
            let mut child = (*(*u).origin).first_child;
            while !child.is_null() {
                let next = (*(*child).origin).sibling;
                Self::dfs_relink(child, last_seen);
                child = next;
            }

            last_seen[c] = prev;
        }
    }

    unsafe fn dfs_sizes(u: *mut Node) -> usize {
        unsafe {
            let mut size = 1;
            let mut child = (*(*u).new).first_child;
            while !child.is_null() {
                let next = (*(*child).new).sibling;
                size += Self::dfs_sizes(child);
                child = next;
            }
            (*u).subtree_size = size;
            size
        }
    }

    fn solve(&mut self) -> Vec<i32> {
        let mut last_seen: [*mut Node; 26] = [null_mut(); 26];

        unsafe {
            let root = *self.nodes;
            Self::dfs_relink(root, &mut last_seen);
            Self::dfs_sizes(root);

            let mut ans = Vec::with_capacity(self.len);
            for i in 0..self.len {
                ans.push((**self.nodes.add(i)).subtree_size as i32);
            }

            ans
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                let ptr = *self.nodes.add(i);
                if !ptr.is_null() {
                    drop(Box::from_raw((*ptr).origin));
                    drop(Box::from_raw((*ptr).new));
                    drop(Box::from_raw(ptr));
                }
            }
            let slice_ptr = ptr::slice_from_raw_parts_mut(self.nodes, self.len);
            drop(Box::from_raw(slice_ptr));
        }
    }
}

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut graph = Graph::from_input(&parent, &s);
        graph.solve()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let parent = vec![-1, 0, 0, 1, 1, 1];
        let s = "abaabc".to_string();

        assert_eq!(
            Solution::find_subtree_sizes(parent, s),
            vec![6, 3, 1, 1, 1, 1]
        );
    }

    #[test]
    fn example2() {
        let parent = vec![-1, 0, 4, 0, 1];
        let s = "abbba".to_string();

        assert_eq!(Solution::find_subtree_sizes(parent, s), vec![5, 2, 1, 1, 1]);
    }

    #[test]
    fn nearest_ancestor_is_restored_between_siblings() {
        let parent = vec![-1, 0, 0, 2];
        let s = "aaba".to_string();

        assert_eq!(Solution::find_subtree_sizes(parent, s), vec![4, 1, 1, 1]);
    }
}
