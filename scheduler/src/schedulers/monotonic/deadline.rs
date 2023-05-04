use crate::{
    log::Log,
    schedulers::{
        CheckSchedulable,
        AddTaskCapabilities,
        SchedulabilityResult,
        SchedulerInterface,
        monotonic::{
            Task,
            GetTasksMut,
            GetTasks,
            AssignPriorities,
            CheckRTA,
        },
    },
};

#[derive(Debug)]
pub struct DeadlineMonotonicScheduler {
    // Tasks With associated priority
    tasks: Vec<(Option<usize>,Task)>,
}


impl DeadlineMonotonicScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        DeadlineMonotonicScheduler {
            tasks: vec![],
        }
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

impl AssignPriorities for DeadlineMonotonicScheduler {
    // Les dues altres funcions les derivem de la implementació default
    fn sort_tasks(&mut self) {
        // Ordenem les tasques de més a menys prioritaria
        self.tasks.sort_by(|(_,a),(_,b)| a.get_deadline().cmp(&b.get_deadline()));
    }
}

// Implementacions default de les diferents funcions de checking
// (Només el RTA és necesari en aquest cas)
impl CheckRTA for DeadlineMonotonicScheduler {}

// Interface per a assegurar a l'usuari que implementem unes certes funcions
impl SchedulerInterface for DeadlineMonotonicScheduler {}

impl CheckSchedulable for DeadlineMonotonicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        let mut log = Log::new();
        
        log.add_event(format!("Asignem prioritats a les tasques"));
        self.sort_n_assign();
        
        // Comprovem el RTA
        log.add_event(format!("Comprovem el Response Time Analysis."));
        let (result_rta, log_rta) = self.check_rta();
        log.append_log(log_rta);
        if result_rta {
            log.add_event(format!("El Response Time Analysis es compleix"));
            return SchedulabilityResult::Schedulable(Some(log));
        }
        log.add_error(format!("El Response Time Analysis ha fallat"));

        // Com que tots els checks han fallat, el sistema no es planificable
        SchedulabilityResult::NotSchedulable(Some(log))
    }
}

impl AddTaskCapabilities for DeadlineMonotonicScheduler {
    fn add_task(&mut self, computing_time: f64, deadline: usize, period: usize) -> Result<(), String> {
        // Error checking
        if period < deadline { return Err(String::from("Period < Deadline")); }
        if computing_time < 0.0 { return Err(String::from("Computing Time < 0")) }

        // Really adding the task
        self.tasks.push((None, Task::new(computing_time, deadline, period)));
        Ok(()) 
    }
}
