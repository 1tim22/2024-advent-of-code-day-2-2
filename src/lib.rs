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
    let mut is_safe: bool = false;

    is_safe = is_safe || (0..list.len()).map(|i| {
        let iterator = list.iter().take(i).chain(list.iter().skip(i + 1));
        iterator.clone().is_sorted() && is_list_interval_safe(iterator.collect::<Vec<&i32>>())
    }).filter(|&value| value)
    .count() > 0;

    is_safe = is_safe || (0..list.len()).map(|i| {
        let iterator = list.iter().rev().take(i).chain(list.iter().rev().skip(i + 1));
        iterator.clone().is_sorted() && is_list_interval_safe(iterator.collect::<Vec<&i32>>())
    }).filter(|&value| value)
    .count() > 0;

    is_safe
}

// Any two adjacent levels differ by at least one and at most three.
fn is_list_interval_safe(list: Vec<&i32>) -> bool {
    list.chunk_by(|&a, &b| {
        let diff: i32 = (a - b).abs();
        diff < 1 || diff > 3
    }).map(|chunk| 1 >= chunk.len())
    .filter(|&value| !value)
    .collect::<Vec<bool>>()
    .is_empty()
}
