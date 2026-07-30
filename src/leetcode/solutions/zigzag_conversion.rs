struct Solution;

struct Node {
    value: char,
    right: Option<usize>,
    zigzag: Option<usize>,
}

struct NodeGraph {
    nodes: Vec<Node>,
    row_heads: Vec<Option<usize>>,
}

impl NodeGraph {
    fn new(s: &str, num_rows: usize) -> Self {
        if s.is_empty() || num_rows <= 0 {
            return Self {
                nodes: Vec::new(),
                row_heads: Vec::new(),
            };
        }

        let actual_rows = num_rows.min(s.len());

        let mut graph = Self {
            nodes: Vec::new(),
            row_heads: vec![None; actual_rows],
        };

        let mut row_tails: Vec<Option<usize>> = vec![None; actual_rows];

        let mut current_row = 0usize;
        let mut going_down = true;

        let mut previous_zigzag: Option<usize> = None;

        for ch in s.chars() {
            let node_index = graph.nodes.len();

            graph.nodes.push(Node {
                value: ch,
                right: None,
                zigzag: None,
            });

            if let Some(previous_index) = previous_zigzag {
                graph.nodes[previous_index].zigzag = Some(node_index);
            }

            previous_zigzag = Some(node_index);

            match row_tails[current_row] {
                Some(tail_index) => graph.nodes[tail_index].right = Some(node_index),
                None => graph.row_heads[current_row] = Some(node_index),
            }

            row_tails[current_row] = Some(node_index);

            if actual_rows > 1 {
                if current_row == 0 {
                    going_down = true;
                } else if current_row == actual_rows - 1 {
                    going_down = false;
                }

                if going_down {
                    current_row += 1;
                } else {
                    current_row -= 1;
                }
            }
        }

        graph
    }

    fn read_by_rows(&self) -> String {
        let mut result = String::with_capacity(self.nodes.len());

        for &head in &self.row_heads {
            let mut current = head;

            while let Some(index) = current {
                result.push(self.nodes[index].value);
                current = self.nodes[index].right;
            }
        }
        result
    }
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let graph = NodeGraph::new(&s, num_rows as usize);

        graph.read_by_rows()
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::solutions::zigzag_conversion::Solution;

    #[test]
    fn example1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;

        assert_eq!("PAHNAPLSIIGYIR".to_string(), Solution::convert(s, num_rows))
    }

    #[test]
    fn example2() {
        let s = "A".to_string();
        let num_rows = 1;

        assert_eq!("A".to_string(), Solution::convert(s, num_rows))
    }

    #[test]
    fn example3() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;

        assert_eq!("PINALSIGYAHRPI".to_string(), Solution::convert(s, num_rows))
    }
}
