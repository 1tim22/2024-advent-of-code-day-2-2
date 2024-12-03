use day_2::*;

#[test]
fn test_safe_reports() {
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

    assert_eq!(count, 2);
}
