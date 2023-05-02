use crate::{schedulers::{
    CheckSchedulable,
    AddTaskCapabilities,
    SchedulerInterface,
    SchedulabilityResult,
}, log::Log};
use super::{
    Task,
    GetTasksMut,
    GetTasks,
    EqualMultipliers,
    AssignPriorities,
    CheckSC1,
    CheckSC2,
    CheckRTA, 
};

#[derive(Debug)]
pub struct RateMonotonicScheduler {
    // With associated priority
    tasks: Vec<(Option<usize>,Task)>,
    log: Log,
}


// Implementacions genèriques
impl RateMonotonicScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        RateMonotonicScheduler {
            tasks: vec![],
            log: Log::new(),
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
    // Les altres dues funcions usen la implementació default
    fn sort_tasks(&mut self) {
        self.tasks.sort_by(|a,b| a.1.get_period().cmp(&b.1.get_period()));
    }
}

// Implementacions default de les diferents funcions de checking
impl CheckSC1 for RateMonotonicScheduler {}
impl CheckSC2 for RateMonotonicScheduler {}
impl CheckRTA for RateMonotonicScheduler {}

impl CheckSchedulable for RateMonotonicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        // Reiniciem el log
        self.log = Log::new();

        self.log.add_event(format!("Igualem els multiplicadors"));
        self.equal_multipliers();
        
        self.log.add_event(format!("Asignem prioritats a les tasques"));
        self.sort_n_assign();

        let (result, log) = self.check_sc1();
        self.log.append_log(log);
        if result {
            return SchedulabilityResult::Schedulable(Some(self.log.clone()));
        }
        
        let (result, log) = self.check_sc2();
        self.log.append_log(log);
        if result {
            return SchedulabilityResult::Schedulable(Some(self.log.clone()));
        }

        let (result, log) = self.check_rta();
        self.log.append_log(log);
        if result {
            return SchedulabilityResult::Schedulable(Some(self.log.clone()));
        }

        SchedulabilityResult::NotSchedulable(Some(self.log.clone()))
    }
}

impl AddTaskCapabilities for RateMonotonicScheduler {
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        // Error checking
        if period < deadline { return Err(String::from("Period < Deadline")); }
        if computing_time < 0.0 { return Err(String::from("Computing Time < 0")) }

        // Really adding the task
        self.tasks.push((None, Task::new(computing_time, deadline, period)));
        Ok(()) 
    }
}

impl SchedulerInterface for RateMonotonicScheduler {}
