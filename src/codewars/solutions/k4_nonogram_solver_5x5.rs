const BLOCKED: u8 = 0;
const FILLED: u8 = 1;
const UNKNOWN: u8 = 3;

fn solve_nonogram((top_clues, left_clues): ([&[u8]; 5], [&[u8]; 5])) -> [[u8; 5]; 5] {
    solve_nonogram_sized::<5>(top_clues, left_clues)
}

fn solve_nonogram_sized<const SIZE: usize>(
    top_clues: [&[u8]; SIZE],
    left_clues: [&[u8]; SIZE],
) -> [[u8; SIZE]; SIZE] {
    let mut nonogram = [[UNKNOWN; SIZE]; SIZE];
    let row_patterns = left_clues.map(patterns_for_clue::<SIZE>);
    let column_patterns = top_clues.map(patterns_for_clue::<SIZE>);

    loop {
        let previous_hash = nonogram_hash(&nonogram);

        apply_line_constraints(&mut nonogram, &row_patterns, &column_patterns);

        if nonogram_hash(&nonogram) == previous_hash {
            break;
        }
    }

    if nonogram.iter().flatten().all(|cell| *cell != UNKNOWN) {
        return nonogram;
    }

    solve_by_search(&row_patterns, &column_patterns).unwrap_or(nonogram)
}

fn apply_line_constraints<const SIZE: usize>(
    nonogram: &mut [[u8; SIZE]; SIZE],
    row_patterns: &[Vec<[u8; SIZE]>; SIZE],
    column_patterns: &[Vec<[u8; SIZE]>; SIZE],
) {
    for row in 0..SIZE {
        let current = nonogram[row];
        let possible_patterns = matching_patterns(&row_patterns[row], &current);
        fill_common_cells(&mut nonogram[row], &possible_patterns);
    }

    for column in 0..SIZE {
        let current = column_values(nonogram, column);
        let possible_patterns = matching_patterns(&column_patterns[column], &current);
        let mut solved_column = current;

        fill_common_cells(&mut solved_column, &possible_patterns);

        for row in 0..SIZE {
            nonogram[row][column] = solved_column[row];
        }
    }
}

fn solve_by_search<const SIZE: usize>(
    row_patterns: &[Vec<[u8; SIZE]>; SIZE],
    column_patterns: &[Vec<[u8; SIZE]>; SIZE],
) -> Option<[[u8; SIZE]; SIZE]> {
    fn search<const SIZE: usize>(
        row: usize,
        nonogram: &mut [[u8; SIZE]; SIZE],
        row_patterns: &[Vec<[u8; SIZE]>; SIZE],
        column_patterns: &[Vec<[u8; SIZE]>; SIZE],
    ) -> bool {
        if row == SIZE {
            return (0..SIZE).all(|column| {
                let current = column_values(nonogram, column);
                column_patterns[column].contains(&current)
            });
        }

        for pattern in &row_patterns[row] {
            nonogram[row] = *pattern;

            if columns_still_possible(nonogram, row, column_patterns)
                && search(row + 1, nonogram, row_patterns, column_patterns)
            {
                return true;
            }
        }

        false
    }

    let mut nonogram = [[UNKNOWN; SIZE]; SIZE];

    if search(0, &mut nonogram, row_patterns, column_patterns) {
        Some(nonogram)
    } else {
        None
    }
}

fn columns_still_possible<const SIZE: usize>(
    nonogram: &[[u8; SIZE]; SIZE],
    current_row: usize,
    column_patterns: &[Vec<[u8; SIZE]>; SIZE],
) -> bool {
    (0..SIZE).all(|column| {
        column_patterns[column]
            .iter()
            .any(|pattern| (0..=current_row).all(|row| nonogram[row][column] == pattern[row]))
    })
}

fn patterns_for_clue<const SIZE: usize>(clue: &[u8]) -> Vec<[u8; SIZE]> {
    fn place_clue<const SIZE: usize>(
        clue: &[u8],
        clue_index: usize,
        start: usize,
        line: &mut [u8; SIZE],
        patterns: &mut Vec<[u8; SIZE]>,
    ) {
        if clue_index == clue.len() {
            patterns.push(*line);
            return;
        }

        let block_len = clue[clue_index] as usize;
        let remaining_width = clue[clue_index + 1..]
            .iter()
            .map(|value| *value as usize)
            .sum::<usize>()
            + clue.len().saturating_sub(clue_index + 1);

        for block_start in start..=SIZE - block_len - remaining_width {
            let mut next_line = *line;

            for cell in next_line.iter_mut().take(block_start).skip(start) {
                *cell = BLOCKED;
            }

            for cell in next_line.iter_mut().skip(block_start).take(block_len) {
                *cell = FILLED;
            }

            let next_start = block_start + block_len + 1;
            if clue_index + 1 == clue.len() {
                for cell in next_line.iter_mut().skip(block_start + block_len) {
                    *cell = BLOCKED;
                }
                patterns.push(next_line);
            } else {
                next_line[block_start + block_len] = BLOCKED;
                place_clue(clue, clue_index + 1, next_start, &mut next_line, patterns);
            }
        }
    }

    let mut patterns = Vec::new();
    let mut line = [BLOCKED; SIZE];

    place_clue(clue, 0, 0, &mut line, &mut patterns);

    patterns
}

fn matching_patterns<const SIZE: usize>(
    patterns: &[[u8; SIZE]],
    current: &[u8; SIZE],
) -> Vec<[u8; SIZE]> {
    patterns
        .iter()
        .copied()
        .filter(|pattern| {
            pattern
                .iter()
                .zip(current)
                .all(|(pattern_cell, current_cell)| {
                    *current_cell == UNKNOWN || *current_cell == *pattern_cell
                })
        })
        .collect()
}

fn fill_common_cells<const SIZE: usize>(line: &mut [u8; SIZE], patterns: &[[u8; SIZE]]) {
    for index in 0..SIZE {
        let first = patterns[0][index];

        if patterns.iter().all(|pattern| pattern[index] == first) {
            line[index] = first;
        }
    }
}

fn column_values<const SIZE: usize>(nonogram: &[[u8; SIZE]; SIZE], column: usize) -> [u8; SIZE] {
    let mut values = [UNKNOWN; SIZE];

    for row in 0..SIZE {
        values[row] = nonogram[row][column];
    }

    values
}

fn nonogram_hash<const SIZE: usize>(nonogram: &[[u8; SIZE]; SIZE]) -> Vec<u8> {
    nonogram.iter().flatten().copied().collect()
}

fn clue_sum(clue: &[u8]) -> u8 {
    clue.len() as u8 - 1 + clue.iter().sum::<u8>()
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn clue_sum_multiple_items() {
        // 1, 0, 1, 1
        assert_eq!(4, clue_sum(&[1, 2]))
    }

    #[test]
    fn test1() {
        assert_eq!(solve_nonogram(CLUES_1), ANS_1);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_nonogram(CLUES_2), ANS_2);
    }

    const CLUES_1: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[4], &[1, 1, 1], &[3], &[1]],
        [&[1], &[2], &[3], &[2, 1], &[4]],
    );

    const ANS_1: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 0, 1, 0],
        [0, 1, 1, 1, 1],
    ];

    const CLUES_2: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1], &[3], &[1], &[3, 1], &[3, 1]],
        [&[3], &[2], &[2, 2], &[1], &[1, 2]],
    );

    const ANS_2: [[u8; 5]; 5] = [
        [0, 0, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 1, 1],
    ];
}
