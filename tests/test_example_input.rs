use day_2::*;

#[test]
fn test_safe_reports_example_input() {
    let count = safe_report_count(
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ],
    );

    assert_eq!(count, 4);
}

#[test]
fn test_safe_reports_peaked_levels() {
    let count = safe_report_count(
        vec![
            vec![2, 4, 6, 9, 10, 9],              // safe
            vec![45, 48, 51, 52, 55, 58, 60, 60], // safe
            vec![50, 49, 47, 50, 49, 48, 47, 42],
            vec![67, 71, 77, 80, 83, 84, 84],
            vec![70, 74, 75, 80, 84],
            vec![77, 74, 71, 69, 64, 67],         // safe
        ],
    );

    assert_eq!(count, 3);
}
