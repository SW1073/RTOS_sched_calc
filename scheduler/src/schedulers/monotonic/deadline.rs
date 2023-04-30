use crate::{
    schedulers::{
        CheckSchedulable,
        SchedulabilityResult,
    },
    log::Log
};
use super::{
    Task,
    GetTasksMut, 
    GetTasks,
    EqualMultipliers,
    AssignPriorities,
    CheckRTA,
};

#[derive(Debug)]
pub struct DeadlineMonotonicScheduler {
    // With associated priority
    tasks: Vec<(Option<usize>,Task)>,
    log: Log,
}


// Implementacions genèriques
impl DeadlineMonotonicScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        DeadlineMonotonicScheduler {
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

impl GetTasksMut for DeadlineMonotonicScheduler {
    fn get_tasks_mut(&mut self) -> std::slice::IterMut<'_, (Option<usize>, Task)> {
        self.tasks.iter_mut()
    }
}

impl GetTasks for DeadlineMonotonicScheduler {
    fn get_tasks(&self) -> &Vec<(Option<usize>, Task)> {
        return &self.tasks;
    }
}

// Deixem la implementacio default per a igualar els multiplicadors
impl EqualMultipliers for DeadlineMonotonicScheduler {
    fn equal_multipliers(&mut self) {
        let max = self.tasks.iter().map(|a| a.1.get_multiplier()).max().unwrap_or(0);
        for t in self.tasks.iter_mut() {
            t.1.set_multiplier(max);
        }
    }
}

// Assignem prioritats al scheduler
impl AssignPriorities for DeadlineMonotonicScheduler {
    fn assign_priorities(&mut self) {
        // Sort vector
        self.tasks.sort_by(|a,b| a.1.get_deadline().cmp(&b.1.get_deadline()));
        // Set priorities based on their position in the ordered vector
        let mut i = self.tasks.len();
        for t in self.tasks.iter_mut() {
            t.0 = Some(i);
            i-=1;
        }
    }
}

// Implementacions default de les diferents funcions de checking
// (Només el RTA és necesari en aquest cas)
impl CheckRTA for DeadlineMonotonicScheduler {}

impl CheckSchedulable for DeadlineMonotonicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        
        self.log.add_event(format!("Igualem els multiplicadors"));
        self.equal_multipliers();

        self.log.add_event(format!("Asignem prioritats a les tasques"));
        self.assign_priorities();

        // Només cal que es compleixi l'RTA
        self.log.add_event(format!("Comprovem el Response Time Analysis."));
        match self.check_rta() {
            true => {
                self.log.add_event(format!("El Response Time Analysis es compleix"));
                SchedulabilityResult::Schedulable(Some(self.log.clone()))
            },
            false => {
                self.log.add_error(format!("El Response Time Analysis no es compleix"));
                SchedulabilityResult::NotSchedulable(Some(self.log.clone()))
            },
        }
    }
}
