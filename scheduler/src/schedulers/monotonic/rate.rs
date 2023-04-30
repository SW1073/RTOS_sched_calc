use crate::schedulers::{
    CheckSchedulable,
    SchedulabilityResult,
};
use super::{
    Task,
    GetTasksMut,
    GetTasks,
    EqualMultipliers,
    AssignPriorities,
    CheckSC1,
    CheckSC2,
    CheckRTA
};

#[derive(Debug)]
pub struct RateMonotonicScheduler {
    // With associated priority
    tasks: Vec<(Option<usize>,Task)>,
}


// Implementacions genèriques
impl RateMonotonicScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        RateMonotonicScheduler {
            tasks: vec![],
        }
    }

    /**
     * Afegeix una nova tasca al planificador
     */
    pub fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        self.tasks.push((None, Task::new(computing_time, deadline, period)));
        Ok(())   
    }
}

impl GetTasksMut for RateMonotonicScheduler {
    fn get_tasks_mut(&mut self) -> std::slice::IterMut<'_, (Option<usize>, Task)> {
        self.tasks.iter_mut()
    }
}

impl GetTasks for RateMonotonicScheduler {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)> {
        return &self.tasks;
    }
}

// Deixem la implementacio default per a igualar els multiplicadors
impl EqualMultipliers for RateMonotonicScheduler { }

// Assignem prioritats al scheduler
impl AssignPriorities for RateMonotonicScheduler {
    fn assign_priorities(&mut self) {
        // Sort vector
        self.tasks.sort_by(|a,b| a.1.get_period().cmp(&b.1.get_period()));
        // Set priorities based on their position in the ordered vector
        let mut i = self.tasks.len();
        for t in self.tasks.iter_mut() {
            t.0 = Some(i);
            i-=1;
        }
    }
}

// Implementacions default de les diferents funcions de checking
impl CheckSC1 for RateMonotonicScheduler {}
impl CheckSC2 for RateMonotonicScheduler {}
impl CheckRTA for RateMonotonicScheduler {}

impl CheckSchedulable for RateMonotonicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        self.equal_multipliers();
        self.assign_priorities();
        // Ha de complir una de les 3 condicions suficients
        match (self.check_sc1(), self.check_sc2(), self.check_rta()) {
            (true, _, _) | (_, true, _) | (_, _, true) => SchedulabilityResult::Schedulable,
            (false, _, _) => SchedulabilityResult::NotSchedulable(String::from("Sufficient Condition 1 not met")),
            (_, false, _) => SchedulabilityResult::NotSchedulable(String::from("Sufficient Condition 2 not met")),
            (_, _, false) => SchedulabilityResult::NotSchedulable(String::from("Response Time Analysis not met")),           
        }
    }
}
