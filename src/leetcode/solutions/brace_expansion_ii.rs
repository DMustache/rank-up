use std::collections::BTreeSet;

struct Solution;

struct Node {
    letter: char,
    sequence: Vec<Node>,
    union: Vec<Node>,
}

impl Node {
    fn from_expression(expression: &str) -> Self {
        let bytes = expression.as_bytes();
        let mut index = 0;
        Self::parse_expression(bytes, &mut index)
    }

    fn parse_expression(bytes: &[u8], index: &mut usize) -> Self {
        let mut alternatives = vec![Self::parse_sequence(bytes, index)];

        while *index < bytes.len() && bytes[*index] == b',' {
            *index += 1;
            alternatives.push(Self::parse_sequence(bytes, index));
        }

        if alternatives.len() == 1 {
            alternatives.pop().unwrap()
        } else {
            Self {
                letter: '\0',
                sequence: Vec::new(),
                union: alternatives,
            }
        }
    }

    fn parse_sequence(bytes: &[u8], index: &mut usize) -> Self {
        let mut nodes = Vec::new();

        while *index < bytes.len() && bytes[*index] != b',' && bytes[*index] != b'}' {
            nodes.push(Self::parse_term(bytes, index));
        }

        if nodes.len() == 1 {
            nodes.pop().unwrap()
        } else {
            Self {
                letter: '\0',
                sequence: nodes,
                union: Vec::new(),
            }
        }
    }

    fn parse_term(bytes: &[u8], index: &mut usize) -> Self {
        if bytes[*index] == b'{' {
            *index += 1;
            let node = Self::parse_expression(bytes, index);
            *index += 1; // '}'
            node
        } else {
            let letter = bytes[*index] as char;
            *index += 1;

            Self {
                letter,
                sequence: Vec::new(),
                union: Vec::new(),
            }
        }
    }

    fn words(&self) -> BTreeSet<String> {
        if !self.union.is_empty() {
            let mut result = BTreeSet::new();

            for child in &self.union {
                result.extend(child.words());
            }

            return result;
        }

        if !self.sequence.is_empty() {
            let mut result = BTreeSet::from([String::new()]);

            for child in &self.sequence {
                let mut combined = BTreeSet::new();

                for prefix in &result {
                    for suffix in child.words() {
                        combined.insert(format!("{prefix}{suffix}"));
                    }
                }

                result = combined;
            }

            return result;
        }

        BTreeSet::from([self.letter.to_string()])
    }
}

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let root = Node::from_expression(&expression);
        let words = root.words();

        let mut result = Vec::with_capacity(words.len());

        for word in words {
            result.push(word);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
            vec!["ac", "ad", "ae", "bc", "bd", "be"]
        );
    }

    #[test]
    fn example2_removes_duplicates_and_sorts_the_result() {
        assert_eq!(
            Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
            vec!["a", "ab", "ac", "z"]
        );
    }
}
