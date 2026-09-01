//! School Timetable / Stundenplanung domain model and solver.

use std::collections::{HashMap, HashSet};

use crate::solver::{Conflict, Constraint, ConstraintSolver, Domain, VariableId};

/// A lesson requiring scheduling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lesson {
    /// Lesson identifier.
    pub id: usize,
    /// Subject name.
    pub subject: String,
    /// Assigned teacher.
    pub teacher: String,
    /// Assigned class group.
    pub class_group: String,
}

/// Assigned schedule slot for a lesson.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleAssignment {
    /// Time slot index (e.g. 0 = Mon 1st period, 1 = Mon 2nd period, etc.).
    pub slot: usize,
    /// Room index.
    pub room: usize,
}

/// Pairwise constraint between two scheduled lessons.
struct LessonPairConstraint {
    var1: VariableId,
    var2: VariableId,
    same_teacher: bool,
    same_class: bool,
}

impl Constraint<ScheduleAssignment> for LessonPairConstraint {
    fn variables(&self) -> Vec<VariableId> {
        vec![self.var1, self.var2]
    }

    fn propagate(
        &self,
        domains: &mut HashMap<VariableId, Domain<ScheduleAssignment>>,
    ) -> Result<bool, Conflict> {
        let mut changed = false;

        let val1_opt = domains.get(&self.var1).and_then(|d| d.singleton());
        let val2_opt = domains.get(&self.var2).and_then(|d| d.singleton());

        if let Some(val1) = val1_opt {
            let same_t = self.same_teacher;
            let same_c = self.same_class;

            if let Some(d2) = domains.get_mut(&self.var2)
                && d2.retain(|val2| {
                    if val1.slot == val2.slot && val1.room == val2.room {
                        return false;
                    }
                    if same_t && val1.slot == val2.slot {
                        return false;
                    }
                    if same_c && val1.slot == val2.slot {
                        return false;
                    }
                    true
                })
            {
                changed = true;
                if d2.is_empty() {
                    return Err(Conflict(self.var2));
                }
            }
        }

        if let Some(val2) = val2_opt {
            let same_t = self.same_teacher;
            let same_c = self.same_class;

            if let Some(d1) = domains.get_mut(&self.var1)
                && d1.retain(|val1| {
                    if val1.slot == val2.slot && val1.room == val2.room {
                        return false;
                    }
                    if same_t && val1.slot == val2.slot {
                        return false;
                    }
                    if same_c && val1.slot == val2.slot {
                        return false;
                    }
                    true
                })
            {
                changed = true;
                if d1.is_empty() {
                    return Err(Conflict(self.var1));
                }
            }
        }

        Ok(changed)
    }
}

/// School Timetable problem instance.
pub struct TimetableProblem {
    /// Lessons to schedule.
    pub lessons: Vec<Lesson>,
    /// Number of available time slots.
    pub total_slots: usize,
    /// Number of available rooms.
    pub total_rooms: usize,
}

impl TimetableProblem {
    /// Creates a new `TimetableProblem`.
    pub fn new(lessons: Vec<Lesson>, total_slots: usize, total_rooms: usize) -> Self {
        Self {
            lessons,
            total_slots,
            total_rooms,
        }
    }

    /// Solves the timetable problem, returning a mapping from Lesson ID to (Slot, Room).
    pub fn solve(&self) -> Option<HashMap<usize, ScheduleAssignment>> {
        let mut possible_assignments = Vec::new();
        for slot in 0..self.total_slots {
            for room in 0..self.total_rooms {
                possible_assignments.push(ScheduleAssignment { slot, room });
            }
        }

        let mut solver = ConstraintSolver::new();
        let mut lesson_vars = Vec::new();

        for _ in &self.lessons {
            let var = solver.add_variable(possible_assignments.clone());
            lesson_vars.push(var);
        }

        for i in 0..self.lessons.len() {
            for j in (i + 1)..self.lessons.len() {
                let same_teacher = self.lessons[i].teacher == self.lessons[j].teacher;
                let same_class = self.lessons[i].class_group == self.lessons[j].class_group;

                solver.add_constraint(LessonPairConstraint {
                    var1: lesson_vars[i],
                    var2: lesson_vars[j],
                    same_teacher,
                    same_class,
                });
            }
        }

        let solution = solver.solve()?;

        let mut timetable = HashMap::new();
        for (i, lesson) in self.lessons.iter().enumerate() {
            let var = lesson_vars[i];
            if let Some(&assignment) = solution.get(&var) {
                timetable.insert(lesson.id, assignment);
            }
        }

        if self.is_valid_solution(&timetable) {
            Some(timetable)
        } else {
            None
        }
    }

    fn is_valid_solution(&self, timetable: &HashMap<usize, ScheduleAssignment>) -> bool {
        let mut teacher_slots: HashSet<(&str, usize)> = HashSet::new();
        let mut class_slots: HashSet<(&str, usize)> = HashSet::new();
        let mut room_slots: HashSet<(usize, usize)> = HashSet::new();

        for lesson in &self.lessons {
            if let Some(&assign) = timetable.get(&lesson.id) {
                if !teacher_slots.insert((&lesson.teacher, assign.slot)) {
                    return false;
                }
                if !class_slots.insert((&lesson.class_group, assign.slot)) {
                    return false;
                }
                if !room_slots.insert((assign.room, assign.slot)) {
                    return false;
                }
            }
        }

        true
    }
}
