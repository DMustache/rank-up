use std::collections::VecDeque;

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut queues: Vec<VecDeque<u32>> = queues
        .iter()
        .map(|v| v.iter().copied().collect::<VecDeque<u32>>())
        .collect();
    let capacity = capacity as usize;
    let top_floor = queues.len() - 1;
    let mut stops = vec![0];
    let mut floor = 0usize;
    let mut going_up = true;
    let mut cage = Vec::with_capacity(capacity);

    while !cage.is_empty() || queues.iter().any(|queue| !queue.is_empty()) {
        let current_floor = floor as u32;
        let leaving = cage.iter().any(|&destination| destination == current_floor);

        cage.retain(|&destination| destination != current_floor);

        if going_up {
            if !has_above(floor, &cage, &queues) && !has_current_direction(floor, true, &queues) {
                going_up = false;
            }
        } else if !has_below(floor, &cage, &queues) && !has_current_direction(floor, false, &queues)
        {
            going_up = true;
        }

        let mut staying_queue = VecDeque::new();
        let waiting = has_current_direction(floor, going_up, &queues);

        while let Some(destination) = queues[floor].pop_front() {
            let wants_current_direction = if going_up {
                destination > current_floor
            } else {
                destination < current_floor
            };

            if wants_current_direction && cage.len() < capacity {
                cage.push(destination);
            } else {
                staying_queue.push_back(destination);
            }
        }

        queues[floor] = staying_queue;

        if (leaving || waiting) && stops.last() != Some(&current_floor) {
            stops.push(current_floor);
        }

        if cage.is_empty() && !queues.iter().any(|queue| !queue.is_empty()) {
            break;
        }

        if going_up {
            floor += 1;
        } else {
            floor = floor.saturating_sub(1);
        }

        if floor == top_floor {
            going_up = false;
        } else if floor == 0 {
            going_up = true;
        }
    }

    if stops.last() != Some(&0) {
        stops.push(0);
    }

    stops
}

fn has_above(floor: usize, cage: &[u32], queues: &[VecDeque<u32>]) -> bool {
    cage.iter()
        .any(|&destination| (destination as usize) > floor)
        || queues
            .iter()
            .enumerate()
            .skip(floor + 1)
            .any(|(_, queue)| !queue.is_empty())
}

fn has_below(floor: usize, cage: &[u32], queues: &[VecDeque<u32>]) -> bool {
    cage.iter()
        .any(|&destination| (destination as usize) < floor)
        || queues
            .iter()
            .enumerate()
            .take(floor)
            .any(|(_, queue)| !queue.is_empty())
}

fn has_current_direction(floor: usize, going_up: bool, queues: &[VecDeque<u32>]) -> bool {
    queues[floor].iter().any(|&destination| {
        if going_up {
            (destination as usize) > floor
        } else {
            (destination as usize) < floor
        }
    })
}

#[cfg(test)]
mod tests {
    use ntest::timeout;

    use super::the_lift;

    fn print_queues(queues: &[Vec<u32>], capacity: u32) -> String {
        let mut result = format!("\nLift capacity = {capacity}\n\n Floor    Queue");
        for (i, q) in queues.iter().enumerate().rev() {
            result.push_str(&format!("\n{i:>4} .... {q:?}"));
        }
        result
    }

    fn do_test(queues: &[Vec<u32>], capacity: u32, expected: &[u32]) {
        let actual = the_lift(queues, capacity);
        assert_eq!(
            actual,
            expected,
            "\nYour result (left) did not match expected output (right) for the given queues:\n{}\n",
            print_queues(queues, capacity)
        );
    }

    #[test]
    fn test_up() {
        do_test(
            &[
                vec![],
                vec![],
                vec![5, 5, 5],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            5,
            &[0, 2, 5, 0],
        );
    }

    #[test]
    fn test_down() {
        do_test(
            &[vec![], vec![], vec![1], vec![], vec![], vec![], vec![]],
            5,
            &[0, 2, 1, 0],
        );
    }
    #[test]
    fn test_up_and_up() {
        do_test(
            &[vec![], vec![3], vec![4], vec![], vec![5], vec![], vec![]],
            5,
            &[0, 1, 2, 3, 4, 5, 0],
        );
    }
    #[test]
    #[timeout(10)]
    fn test_down_and_down() {
        do_test(
            &[vec![], vec![0], vec![], vec![], vec![2], vec![3], vec![]],
            5,
            &[0, 5, 4, 3, 2, 1, 0],
        );
    }
}
