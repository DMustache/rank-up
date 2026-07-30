fn find_min_index<T: Ord>(arr: &[T]) -> usize {
    let mut smallest = &arr[0];
    let mut smallest_index = 0;
    for i in 1..arr.len() {
        if &arr[i] < smallest {
            smallest = &arr[i];
            smallest_index = i;
        }
    }
    smallest_index
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted_end = 0;
    for i in 0..arr.len() {
        let smallest_index = find_min_index(&arr[sorted_end..]);
        arr.swap(sorted_end + smallest_index, i);
        sorted_end += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }
}
