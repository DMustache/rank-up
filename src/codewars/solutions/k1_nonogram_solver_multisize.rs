use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

type Clues<'a> = &'a [&'a [u8]];

const BLOCKED: u8 = 0;
const FILLED: u8 = 1;

pub fn solve((top_clues, left_clues): (Clues, Clues), width: usize, height: usize) -> Vec<Vec<u8>> {
    let solver = Solver::new(top_clues, left_clues, width, height);
    let mut failed_states = HashSet::new();

    match search(solver, &mut failed_states) {
        Some(solution) => solution.to_board(),
        None => vec![vec![BLOCKED; width]; height],
    }
}

#[derive(Clone)]
struct Solver {
    width: usize,
    height: usize,
    row_full_mask: u128,
    column_full_mask: u128,
    row_filled: Vec<u128>,
    row_blocked: Vec<u128>,
    column_filled: Vec<u128>,
    column_blocked: Vec<u128>,
    row_active: Vec<Vec<usize>>,
    column_active: Vec<Vec<usize>>,
    patterns: Rc<PatternStore>,
}

struct PatternStore {
    rows: Vec<Vec<u128>>,
    columns: Vec<Vec<u128>>,
}

enum Branch {
    Row(usize),
    Column(usize),
}

impl Solver {
    fn new(top_clues: Clues, left_clues: Clues, width: usize, height: usize) -> Self {
        let rows: Vec<Vec<u128>> = left_clues
            .iter()
            .map(|clue| patterns_for_clue(clue, width))
            .collect();
        let columns: Vec<Vec<u128>> = top_clues
            .iter()
            .map(|clue| patterns_for_clue(clue, height))
            .collect();
        let row_active = active_indexes(&rows);
        let column_active = active_indexes(&columns);

        Self {
            width,
            height,
            row_full_mask: full_mask(width),
            column_full_mask: full_mask(height),
            row_filled: vec![0; height],
            row_blocked: vec![0; height],
            column_filled: vec![0; width],
            column_blocked: vec![0; width],
            row_active,
            column_active,
            patterns: Rc::new(PatternStore { rows, columns }),
        }
    }

    fn propagate(&mut self) -> bool {
        let mut row_queue: VecDeque<usize> = (0..self.height).collect();
        let mut column_queue: VecDeque<usize> = (0..self.width).collect();
        let mut row_queued = vec![true; self.height];
        let mut column_queued = vec![true; self.width];

        while !row_queue.is_empty() || !column_queue.is_empty() {
            while let Some(row) = row_queue.pop_front() {
                row_queued[row] = false;
                let before_len = self.row_active[row].len();
                let filled = self.row_filled[row];
                let blocked = self.row_blocked[row];
                let patterns = &self.patterns.rows[row];

                self.row_active[row].retain(|pattern_index| {
                    pattern_matches_known(patterns[*pattern_index], filled, blocked)
                });

                if self.row_active[row].is_empty() {
                    return false;
                }

                let (common_filled, common_blocked) =
                    common_known_masks(patterns, &self.row_active[row], self.row_full_mask);

                match self.apply_row_known(row, common_filled, common_blocked) {
                    Some(changed_mask) => {
                        enqueue_mask_bits(changed_mask, &mut column_queue, &mut column_queued);
                    }
                    None => return false,
                }

                if before_len != self.row_active[row].len() {
                    enqueue_mask_bits(
                        self.row_filled[row] | self.row_blocked[row],
                        &mut column_queue,
                        &mut column_queued,
                    );
                }
            }

            while let Some(column) = column_queue.pop_front() {
                column_queued[column] = false;
                let before_len = self.column_active[column].len();
                let filled = self.column_filled[column];
                let blocked = self.column_blocked[column];
                let patterns = &self.patterns.columns[column];

                self.column_active[column].retain(|pattern_index| {
                    pattern_matches_known(patterns[*pattern_index], filled, blocked)
                });

                if self.column_active[column].is_empty() {
                    return false;
                }

                let (common_filled, common_blocked) = common_known_masks(
                    patterns,
                    &self.column_active[column],
                    self.column_full_mask,
                );

                match self.apply_column_known(column, common_filled, common_blocked) {
                    Some(changed_mask) => {
                        enqueue_mask_bits(changed_mask, &mut row_queue, &mut row_queued);
                    }
                    None => return false,
                }

                if before_len != self.column_active[column].len() {
                    enqueue_mask_bits(
                        self.column_filled[column] | self.column_blocked[column],
                        &mut row_queue,
                        &mut row_queued,
                    );
                }
            }
        }

        true
    }

    fn apply_row_known(
        &mut self,
        row: usize,
        filled_mask: u128,
        blocked_mask: u128,
    ) -> Option<u128> {
        if filled_mask & self.row_blocked[row] != 0 || blocked_mask & self.row_filled[row] != 0 {
            return None;
        }

        let new_filled = filled_mask & !self.row_filled[row];
        let new_blocked = blocked_mask & !self.row_blocked[row];

        self.row_filled[row] |= filled_mask;
        self.row_blocked[row] |= blocked_mask;

        let row_bit = bit(row);
        let mut filled_cells = new_filled;

        while filled_cells != 0 {
            let column = filled_cells.trailing_zeros() as usize;
            let row_bit = bit(row);

            if self.column_blocked[column] & row_bit != 0 {
                return None;
            }

            self.column_filled[column] |= row_bit;
            filled_cells &= filled_cells - 1;
        }

        let mut blocked_cells = new_blocked;

        while blocked_cells != 0 {
            let column = blocked_cells.trailing_zeros() as usize;

            if self.column_filled[column] & row_bit != 0 {
                return None;
            }

            self.column_blocked[column] |= row_bit;
            blocked_cells &= blocked_cells - 1;
        }

        Some(new_filled | new_blocked)
    }

    fn apply_column_known(
        &mut self,
        column: usize,
        filled_mask: u128,
        blocked_mask: u128,
    ) -> Option<u128> {
        if filled_mask & self.column_blocked[column] != 0
            || blocked_mask & self.column_filled[column] != 0
        {
            return None;
        }

        let new_filled = filled_mask & !self.column_filled[column];
        let new_blocked = blocked_mask & !self.column_blocked[column];

        self.column_filled[column] |= filled_mask;
        self.column_blocked[column] |= blocked_mask;

        let column_bit = bit(column);
        let mut filled_cells = new_filled;

        while filled_cells != 0 {
            let row = filled_cells.trailing_zeros() as usize;

            if self.row_blocked[row] & column_bit != 0 {
                return None;
            }

            self.row_filled[row] |= column_bit;
            filled_cells &= filled_cells - 1;
        }

        let mut blocked_cells = new_blocked;

        while blocked_cells != 0 {
            let row = blocked_cells.trailing_zeros() as usize;

            if self.row_filled[row] & column_bit != 0 {
                return None;
            }

            self.row_blocked[row] |= column_bit;
            blocked_cells &= blocked_cells - 1;
        }

        Some(new_filled | new_blocked)
    }

    fn set_row(&mut self, row: usize, pattern_index: usize) -> bool {
        let pattern = self.patterns.rows[row][pattern_index];

        self.row_active[row].clear();
        self.row_active[row].push(pattern_index);

        match self.apply_row_known(row, pattern, self.row_full_mask ^ pattern) {
            Some(_) => true,
            None => false,
        }
    }

    fn set_column(&mut self, column: usize, pattern_index: usize) -> bool {
        let pattern = self.patterns.columns[column][pattern_index];

        self.column_active[column].clear();
        self.column_active[column].push(pattern_index);

        match self.apply_column_known(column, pattern, self.column_full_mask ^ pattern) {
            Some(_) => true,
            None => false,
        }
    }

    fn is_solved(&self) -> bool {
        self.row_filled
            .iter()
            .zip(&self.row_blocked)
            .all(|(filled, blocked)| (*filled | *blocked) == self.row_full_mask)
    }

    fn next_branch(&self) -> Option<Branch> {
        let mut best: Option<((usize, usize), Branch)> = None;

        for row in 0..self.height {
            let count = self.row_active[row].len();
            let known_cells = (self.row_filled[row] | self.row_blocked[row]).count_ones() as usize;
            let score = (count, self.width - known_cells);

            if count > 1
                && best
                    .as_ref()
                    .map_or(true, |(best_score, _)| score < *best_score)
            {
                best = Some((score, Branch::Row(row)));
            }
        }

        for column in 0..self.width {
            let count = self.column_active[column].len();
            let known_cells =
                (self.column_filled[column] | self.column_blocked[column]).count_ones() as usize;
            let score = (count, self.height - known_cells);

            if count > 1
                && best
                    .as_ref()
                    .map_or(true, |(best_score, _)| score < *best_score)
            {
                best = Some((score, Branch::Column(column)));
            }
        }

        best.map(|(_, branch)| branch)
    }

    fn to_board(&self) -> Vec<Vec<u8>> {
        let mut board = vec![vec![BLOCKED; self.width]; self.height];

        for row in 0..self.height {
            for column in 0..self.width {
                if self.row_filled[row] & bit(column) != 0 {
                    board[row][column] = FILLED;
                }
            }
        }

        board
    }

    fn state_key(&self) -> Vec<u128> {
        let mut key = Vec::with_capacity(self.height * 2);

        key.extend(self.row_filled.iter().copied());
        key.extend(self.row_blocked.iter().copied());

        key
    }
}

fn search(mut solver: Solver, failed_states: &mut HashSet<Vec<u128>>) -> Option<Solver> {
    if !solver.propagate() {
        return None;
    }

    if solver.is_solved() {
        return Some(solver);
    }

    let state_key = solver.state_key();

    if failed_states.contains(&state_key) {
        return None;
    }

    match solver.next_branch() {
        Some(Branch::Row(row)) => {
            let pattern_indexes = solver.row_active[row].clone();

            for pattern_index in pattern_indexes {
                let mut next_solver = solver.clone();

                if next_solver.set_row(row, pattern_index) {
                    if let Some(solution) = search(next_solver, failed_states) {
                        return Some(solution);
                    }
                }
            }
        }
        Some(Branch::Column(column)) => {
            let pattern_indexes = solver.column_active[column].clone();

            for pattern_index in pattern_indexes {
                let mut next_solver = solver.clone();

                if next_solver.set_column(column, pattern_index) {
                    if let Some(solution) = search(next_solver, failed_states) {
                        return Some(solution);
                    }
                }
            }
        }
        None => {}
    }

    failed_states.insert(state_key);

    None
}

fn patterns_for_clue(clue: &[u8], size: usize) -> Vec<u128> {
    fn place_clue(
        clue: &[u8],
        clue_index: usize,
        start: usize,
        size: usize,
        mask: u128,
        patterns: &mut Vec<u128>,
    ) {
        if clue_index == clue.len() {
            patterns.push(mask);
            return;
        }

        let block_len = clue[clue_index] as usize;
        let remaining_width = clue[clue_index + 1..]
            .iter()
            .map(|value| *value as usize)
            .sum::<usize>()
            + clue.len().saturating_sub(clue_index + 1);

        if start + block_len + remaining_width > size {
            return;
        }

        for block_start in start..=size - block_len - remaining_width {
            let next_mask = mask | run_mask(block_start, block_len);
            let next_start = block_start + block_len + 1;

            place_clue(clue, clue_index + 1, next_start, size, next_mask, patterns);
        }
    }

    if clue.is_empty() {
        return vec![0];
    }

    let mut patterns = Vec::new();
    place_clue(clue, 0, 0, size, 0, &mut patterns);
    patterns
}

fn pattern_matches_known(pattern: u128, filled: u128, blocked: u128) -> bool {
    pattern & filled == filled && pattern & blocked == 0
}

fn common_known_masks(patterns: &[u128], active: &[usize], full_mask: u128) -> (u128, u128) {
    let mut common_filled = full_mask;
    let mut possible_filled = 0;

    for pattern_index in active {
        let pattern = patterns[*pattern_index];
        common_filled &= pattern;
        possible_filled |= pattern;
    }

    (common_filled, full_mask ^ possible_filled)
}

fn active_indexes(patterns: &[Vec<u128>]) -> Vec<Vec<usize>> {
    patterns
        .iter()
        .map(|line_patterns| (0..line_patterns.len()).collect())
        .collect()
}

fn enqueue_mask_bits(mask: u128, queue: &mut VecDeque<usize>, queued: &mut [bool]) {
    let mut cells = mask;

    while cells != 0 {
        let index = cells.trailing_zeros() as usize;

        if !queued[index] {
            queued[index] = true;
            queue.push_back(index);
        }

        cells &= cells - 1;
    }
}

const fn full_mask(size: usize) -> u128 {
    if size == 128 {
        u128::MAX
    } else {
        (1u128 << size) - 1
    }
}

const fn run_mask(start: usize, len: usize) -> u128 {
    if len == 0 {
        0
    } else if len == 128 {
        u128::MAX
    } else {
        ((1u128 << len) - 1) << start
    }
}

const fn bit(index: usize) -> u128 {
    1u128 << index
}

#[cfg(test)]
mod example_tests {
    use super::solve;

    macro_rules! test_solve {
        ($name:ident, $idx:literal) => {
            #[test]
            fn $name() {
                let (clues, expected) = PUZZLES[$idx];
                let actual = solve(clues, clues.0.len(), clues.1.len());
                assert_eq!(
                    actual, expected,
                    "\nYour result (left) did not match the expected output (right)"
                );
            }
        };
    }

    test_solve!(_1_solve_5_x_5_puzzle, 0);
    test_solve!(_2_solve_6_x_11_puzzle, 1);
    test_solve!(_3_solve_20_x_20_puzzle, 2);

    type Clues = &'static [&'static [u8]];
    type Solution = &'static [&'static [u8]];

    const PUZZLES: [((Clues, Clues), Solution); 3] = [
        (
            (
                &[&[1, 1], &[4], &[1, 1, 1], &[3], &[1]],
                &[&[1], &[2], &[3], &[2, 1], &[4]],
            ),
            &[
                &[0, 0, 1, 0, 0],
                &[1, 1, 0, 0, 0],
                &[0, 1, 1, 1, 0],
                &[1, 1, 0, 1, 0],
                &[0, 1, 1, 1, 1],
            ],
        ),
        (
            (
                &[&[3], &[4], &[2, 2, 2], &[2, 4, 2], &[6], &[3]],
                &[
                    &[4],
                    &[6],
                    &[2, 2],
                    &[2, 2],
                    &[2],
                    &[2],
                    &[2],
                    &[2],
                    &[],
                    &[2],
                    &[2],
                ],
            ),
            &[
                &[0, 1, 1, 1, 1, 0],
                &[1, 1, 1, 1, 1, 1],
                &[1, 1, 0, 0, 1, 1],
                &[1, 1, 0, 0, 1, 1],
                &[0, 0, 0, 1, 1, 0],
                &[0, 0, 0, 1, 1, 0],
                &[0, 0, 1, 1, 0, 0],
                &[0, 0, 1, 1, 0, 0],
                &[0, 0, 0, 0, 0, 0],
                &[0, 0, 1, 1, 0, 0],
                &[0, 0, 1, 1, 0, 0],
            ],
        ),
        (
            (
                &[
                    &[1, 1, 3],
                    &[3, 2, 1, 3],
                    &[2, 2],
                    &[3, 6, 3],
                    &[3, 8, 2],
                    &[15],
                    &[8, 5],
                    &[15],
                    &[7, 1, 4, 2],
                    &[7, 9],
                    &[6, 4, 2],
                    &[2, 1, 5, 4],
                    &[6, 4],
                    &[2, 6],
                    &[2, 5],
                    &[5, 2, 1],
                    &[6, 1],
                    &[3, 1],
                    &[1, 4, 2, 1],
                    &[2, 2, 2, 2],
                ],
                &[
                    &[2, 1, 1],
                    &[3, 4, 2],
                    &[4, 4, 2],
                    &[8, 3],
                    &[7, 2, 2],
                    &[7, 5],
                    &[9, 4],
                    &[8, 2, 3],
                    &[7, 1, 1],
                    &[6, 2],
                    &[5, 3],
                    &[3, 6, 3],
                    &[2, 9, 2],
                    &[1, 8],
                    &[1, 6, 1],
                    &[3, 1, 6],
                    &[5, 5],
                    &[1, 3, 8],
                    &[1, 2, 6, 1],
                    &[1, 1, 1, 3, 2],
                ],
            ),
            &[
                &[1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                &[0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
                &[1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0],
                &[0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                &[0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1],
                &[0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
                &[0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0],
                &[0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0],
                &[0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
                &[0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1],
                &[1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
                &[1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                &[1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
                &[0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                &[0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
                &[0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
                &[0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1],
            ],
        ),
    ];
}
