use crate::schedulers::CheckSchedulable;
use super::{
    Task,
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


// Deixem la implementacio default per a igualar els multiplicadors
impl EqualMultipliers for RateMonotonicScheduler {
    fn equal_multipliers(&mut self) {
        let max = self.tasks.iter().map(|a| a.1.get_multiplier()).max().unwrap_or(0);
        for t in self.tasks.iter_mut() {
            t.1.set_multiplier(max);
        }
    }
}

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


impl CheckSC1 for RateMonotonicScheduler {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)> {
        return &self.tasks;
    }
}
impl CheckSC2 for RateMonotonicScheduler {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)> {
        return &self.tasks;
    }
}
impl CheckRTA for RateMonotonicScheduler {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)> {
        return &self.tasks;
    }
}

impl CheckSchedulable for RateMonotonicScheduler {
    fn is_schedulable(&mut self) -> bool {
        self.equal_multipliers();
        self.assign_priorities();
        // Ha de complir una de les 3 condicions suficients
        return self.check_sc1() || self.check_sc2() || self.check_rta();
    }
}
