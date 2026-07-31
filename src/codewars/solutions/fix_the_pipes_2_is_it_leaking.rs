#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Node {
    links: Option<Vec<Direction>>,
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '┗' => Self {
                links: Some(vec![Direction::Up, Direction::Right]),
            },
            '┓' => Self {
                links: Some(vec![Direction::Left, Direction::Down]),
            },
            '┏' => Self {
                links: Some(vec![Direction::Right, Direction::Down]),
            },
            '┛' => Self {
                links: Some(vec![Direction::Left, Direction::Up]),
            },
            '━' => Self {
                links: Some(vec![Direction::Left, Direction::Right]),
            },
            '┃' => Self {
                links: Some(vec![Direction::Up, Direction::Down]),
            },
            '┣' => Self {
                links: Some(vec![Direction::Up, Direction::Down, Direction::Right]),
            },
            '┫' => Self {
                links: Some(vec![Direction::Up, Direction::Down, Direction::Left]),
            },
            '┳' => Self {
                links: Some(vec![Direction::Left, Direction::Right, Direction::Down]),
            },
            '┻' => Self {
                links: Some(vec![Direction::Left, Direction::Right, Direction::Up]),
            },
            '╋' => Self {
                links: Some(vec![
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                ]),
            },
            _ => Self { links: None },
        }
    }
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    let grid = pipe_map
        .iter()
        .map(|row| {
            row.chars()
                .map(Node::from)
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let mut queue = VecDeque::new();
    let mut visited = grid
        .iter()
        .map(|row| vec![false; row.len()])
        .collect::<Vec<Vec<bool>>>();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, node) in row.iter().enumerate() {
            let Some(links) = &node.links else {
                continue;
            };

            if links
                .iter()
                .any(|direction| neighbor(&grid, row_index, column_index, *direction).is_none())
            {
                visited[row_index][column_index] = true;
                queue.push_back((row_index, column_index));
            }
        }
    }

    while let Some((row_index, column_index)) = queue.pop_front() {
        let Some(links) = &grid[row_index][column_index].links else {
            continue;
        };

        for direction in links {
            let Some((neighbor_row, neighbor_column)) =
                neighbor(&grid, row_index, column_index, *direction)
            else {
                continue;
            };

            let Some(neighbor_links) = &grid[neighbor_row][neighbor_column].links else {
                return false;
            };

            if !neighbor_links.contains(&direction.opposite()) {
                return false;
            }

            if !visited[neighbor_row][neighbor_column] {
                visited[neighbor_row][neighbor_column] = true;
                queue.push_back((neighbor_row, neighbor_column));
            }
        }
    }

    true
}

fn neighbor(
    grid: &[Vec<Node>],
    row_index: usize,
    column_index: usize,
    direction: Direction,
) -> Option<(usize, usize)> {
    let position = match direction {
        Direction::Up => row_index
            .checked_sub(1)
            .map(|neighbor_row| (neighbor_row, column_index)),
        Direction::Down => Some((row_index + 1, column_index)),
        Direction::Left => column_index
            .checked_sub(1)
            .map(|neighbor_column| (row_index, neighbor_column)),
        Direction::Right => Some((row_index, column_index + 1)),
    }?;

    grid.get(position.0)
        .and_then(|row| row.get(position.1))
        .map(|_| position)
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

fn run_test(pmap: &[&str], answer: bool) {
    let test_result = check_pipe(pmap);
    assert!(
        test_result == answer,
        "Output: {}; expected value: {}; for input:\n{}\n",
        test_result,
        answer,
        pmap.join("\n")
    );
}

#[cfg(test)]
mod sample_tests {

    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in &TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }

    const TEST_CASES: [([&str; 3], bool); 7] = [
        (["╋━━┓", "┃..┃", "┛..┣"], true),
        (["...┏", "┃..┃", "┛..┣"], false),
        (["...┏", "...┃", "┛..┣"], false),
        (["...┏", "...┃", "┓..┣"], true),
        (["╋", "╋", "╋"], true),
        (["╋....", "┃..┛.", "┃...."], false),
        (["....", ".┛┛.", "...."], true),
    ];
}
use std::collections::VecDeque;
