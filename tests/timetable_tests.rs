use pathwise::scheduling::{Lesson, TimetableProblem};

#[test]
fn test_school_timetable_solver() {
    let lessons = vec![
        Lesson {
            id: 1,
            subject: "Math".to_string(),
            teacher: "Mr. Smith".to_string(),
            class_group: "Class 10A".to_string(),
        },
        Lesson {
            id: 2,
            subject: "Physics".to_string(),
            teacher: "Mr. Smith".to_string(), // Same teacher, different lesson
            class_group: "Class 10B".to_string(),
        },
        Lesson {
            id: 3,
            subject: "English".to_string(),
            teacher: "Mrs. Davis".to_string(),
            class_group: "Class 10A".to_string(), // Same class, different lesson
        },
    ];

    let problem = TimetableProblem::new(lessons, 4, 2);
    let timetable = problem
        .solve()
        .expect("School timetable solution should exist");

    assert_eq!(timetable.len(), 3);

    let l1_assign = timetable.get(&1).unwrap();
    let l2_assign = timetable.get(&2).unwrap();
    let l3_assign = timetable.get(&3).unwrap();

    // Teacher Mr. Smith cannot teach L1 and L2 at the same slot
    assert_ne!(l1_assign.slot, l2_assign.slot);

    // Class 10A cannot attend L1 and L3 at the same slot
    assert_ne!(l1_assign.slot, l3_assign.slot);
}
