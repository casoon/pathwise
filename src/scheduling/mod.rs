//! Scheduling and Timetabling Framework.
//!
//! Provides interval, activity, resource models, and school timetabling solvers.

pub mod activity;
pub mod resource;
pub mod timetable;

pub use activity::{Activity, Interval};
pub use resource::{CumulativeResource, UnaryResource};
pub use timetable::{Lesson, ScheduleAssignment, TimetableProblem};
