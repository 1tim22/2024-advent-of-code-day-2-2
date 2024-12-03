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
            vec![7, 6, 4, 6, 7],
            vec![1, 2, 7, 2, 1],
            vec![9, 7, 5, 6, 8],
            vec![1, 3, 2, 4, 5],
        ],
    );

    assert_eq!(count, 1);
}
