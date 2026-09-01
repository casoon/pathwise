use pathwise::scheduling::{Activity, Interval, UnaryResource};

#[test]
fn test_interval_overlaps() {
    let i1 = Interval::new(0, 5);
    let i2 = Interval::new(4, 8);
    let i3 = Interval::new(5, 10);

    assert!(i1.overlaps(&i2));
    assert!(!i1.overlaps(&i3));
    assert_eq!(i1.duration(), 5);
}

#[test]
fn test_unary_resource_no_overlap() {
    let res = UnaryResource::new("Room1");
    let valid_intervals = vec![Interval::new(0, 2), Interval::new(2, 4)];
    let invalid_intervals = vec![Interval::new(0, 3), Interval::new(2, 5)];

    assert!(res.validate_no_overlap(&valid_intervals));
    assert!(!res.validate_no_overlap(&invalid_intervals));
}

#[test]
fn test_activity_construction() {
    let act = Activity::new(1, 2, 0, 10);
    assert_eq!(act.duration, 2);
    assert_eq!(act.release_time, 0);
    assert_eq!(act.deadline, 10);
}
