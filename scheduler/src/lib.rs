pub mod schedulers;
mod task;

pub enum SchedulabilityResult {
    Schedulable,
    NotSchedulable(String), // Reason why it is not schedulable
    Undetermined,
}