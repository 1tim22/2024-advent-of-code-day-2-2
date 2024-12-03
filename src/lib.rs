pub fn safe_report_count(matrix: Vec<Vec<i32>>) -> i32 {
    matrix.iter()
        .map(|list| is_list_safe(list))
        .filter(|&value| value)
        .collect::<Vec<bool>>()
        .len() as i32
}

// A "safe list" is one where:
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
fn is_list_safe(list: &Vec<i32>) -> bool {
    (is_list_ascending(list) || is_list_descending(list))
        && is_list_interval_safe(list)
}

fn is_list_ascending(list: &Vec<i32>) -> bool {
    list.iter().is_sorted()
}

fn is_list_descending(list: &Vec<i32>) -> bool {
    list.iter().rev().is_sorted()
}

// Any two adjacent levels differ by at least one and at most three.
fn is_list_interval_safe(list: &Vec<i32>) -> bool {
    list.chunk_by(|&a, &b| {
        let diff: i32 = (a - b).abs();
        diff < 1 || diff > 3
    }).map(|chunk| 1 >= chunk.len())
    .filter(|&value| !value)
    .collect::<Vec<bool>>()
    .is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_list_ascending() {
        assert!(is_list_ascending(&vec![2, 3, 5, 10]));
        assert!(!is_list_ascending(&vec![13, 12, 11, 4, 0]));
        assert!(is_list_ascending(&vec![13, 13, 13, 13, 13, 13]));
    }

    #[test]
    fn test_is_list_descending() {
        assert!(is_list_descending(&vec![13, 12, 11, 4, 0]));
        assert!(!is_list_descending(&vec![2, 3, 5, 10]));
        assert!(is_list_ascending(&vec![13, 13, 13, 13, 13, 13]));
    }

    #[test]
    fn test_is_list_interval_safe() {
        assert!(is_list_interval_safe(&vec![13, 12, 11, 9, 7]));
        assert!(!is_list_interval_safe(&vec![0, 3, 10, 12, 12, 12, 16, 17]));
        assert!(!is_list_interval_safe(&vec![13, 13, 13, 13, 13, 13]));
    }
}
