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
// If removing a single level from an unsafe report would make it safe, the report instead counts as safe.
fn is_list_safe(list: &Vec<i32>) -> bool {

    let mut is_sorted: bool = false; // list.iter().is_sorted() || list.iter().rev().is_sorted();
    let mut is_safe: bool = false;

    // Ascending with fault tolerance of 1
    for i in 0..list.len() - 1 {
        is_sorted = is_sorted || list.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, &value)| value).is_sorted();
        _ = is_sorted && break;
    }

    // Descending with fault tolerance of 1
    for i in 0..list.len() - 1 {
        is_sorted = is_sorted || list.iter().rev().enumerate().filter(|(j, _)| *j != i).map(|(_, &value)| value).is_sorted();
        _ = is_sorted && break;
    }

    // Enforce level distance with a fault tolerance of 1
    for i in 0..list.len() - 1 {
        is_safe = is_list_interval_safe(list.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, &value)| value).collect::<Vec<i32>>());
        _ = is_safe && break;
    }

    is_sorted && is_safe
}

// Any two adjacent levels differ by at least one and at most three.
fn is_list_interval_safe(list: Vec<i32>) -> bool {
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
    #[ignore]
    fn test_is_list_interval_safe() {
        assert!(is_list_interval_safe(vec![13, 12, 11, 9, 7]));
        assert!(!is_list_interval_safe(vec![0, 3, 10, 12, 12, 12, 16, 17]));
        assert!(!is_list_interval_safe(vec![13, 13, 13, 13, 13, 13]));
    }
}
