pub mod schedulers;
pub mod log;
mod task;

use log::Log;

pub enum SchedulabilityResult {
    Schedulable(Option<Log>),
    NotSchedulable(Option<Log>),
    Undetermined(Option<Log>),
}