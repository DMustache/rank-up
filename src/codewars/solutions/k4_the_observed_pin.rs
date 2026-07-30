use std::collections::HashMap;

use itertools::Itertools;

fn build_neighbours() -> HashMap<char, Vec<char>> {
    static NUMPAD: [[char; 3]; 4] = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
        [' ', '0', ' '],
    ];
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
    let mut map = HashMap::new();

    let active_cells = NUMPAD
        .iter()
        .enumerate()
        .flat_map(|(row, row_slice)| {
            row_slice
                .iter()
                .enumerate()
                .map(move |(column, &val)| (val, row, column))
        })
        .filter(|(value, _, _)| value.is_ascii_digit());

    for (char_num, row, column) in active_cells {
        let mut neighbours = Vec::new();

        for (dirrection_row, dirrection_collumn) in directions.iter() {
            let Ok(neighbour_row) = usize::try_from(row as i8 + dirrection_row) else {
                continue;
            };
            let Ok(neighbour_column) = usize::try_from(column as i8 + dirrection_collumn) else {
                continue;
            };

            if let Some(button) = NUMPAD
                .get(neighbour_row)
                .and_then(|some_row| some_row.get(neighbour_column))
                .filter(|button_predict| button_predict.is_ascii_digit())
            {
                neighbours.push(*button);
            }
        }
        map.insert(char_num, neighbours);
    }

    map
}

fn get_pins(observed: &str) -> Vec<String> {
    let neighbours_map = build_neighbours();

    let result: Vec<&Vec<char>> = observed
        .chars()
        .map(|dight| &neighbours_map[&dight])
        .collect();

    result
        .iter()
        .map(|value| value.iter())
        .multi_cartesian_product()
        .map(|combination| combination.into_iter().collect::<String>())
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
