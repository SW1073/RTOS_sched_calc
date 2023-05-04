use crate::{
    log::Log,
    schedulers::{
        CheckSchedulable,
        AddTaskCapabilities,
        SchedulerInterface,
        SchedulabilityResult,
        monotonic::{
            Task,
            GetTasksMut,
            GetTasks,
            AssignPriorities,
            CheckSC1,
            CheckSC2,
            CheckRTA, 
        },
    },
};

#[derive(Debug)]
pub struct RateMonotonicScheduler {
    // With associated priority
    tasks: Vec<(Option<usize>,Task)>,
}


impl RateMonotonicScheduler {
    /**
     * Constructora default
     */
    pub fn new() -> Self {
        RateMonotonicScheduler {
            tasks: vec![],
        }
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


impl AssignPriorities for RateMonotonicScheduler {
    // Les altres dues funcions usen la implementació default
    fn sort_tasks(&mut self) {
        self.tasks.sort_by(|(_,a),(_,b)| a.get_period().cmp(&b.get_period()));
    }
}

// Implementacions default de les diferents funcions de checking
impl CheckSC1 for RateMonotonicScheduler {}
impl CheckSC2 for RateMonotonicScheduler {}
impl CheckRTA for RateMonotonicScheduler {}

// Interface per a assegurar a l'usuari que implementem unes certes funcions
impl SchedulerInterface for RateMonotonicScheduler {}

impl CheckSchedulable for RateMonotonicScheduler {
    fn is_schedulable(&mut self) -> SchedulabilityResult {
        // Creem un log
        let mut log = Log::new();

        // Ordenem
        log.add_event(format!("Asignem prioritats a les tasques"));
        self.sort_n_assign();

        // Comprovem la SC1
        log.add_event(format!("Comprovem la Sufficient Condition 1"));
        let (result_sc1, log_sc1) = self.check_sc1();
        log.append_log(log_sc1);
        if result_sc1 {
            log.add_event(format!("La Sufficient Condition 1 es compleix"));
            return SchedulabilityResult::Schedulable(Some(log));
        }
        log.add_error(format!("La Sufficient Condition 1 ha fallat"));
        
        // Comprovem la SC2
        log.add_event(format!("Comprovem la Sufficient Condition 2"));
        let (result_sc2, log_sc2) = self.check_sc2();
        log.append_log(log_sc2);
        if result_sc2 {
            log.add_event(format!("La Sufficient Condition 2 es compleix"));
            return SchedulabilityResult::Schedulable(Some(log));
        }
        log.add_error(format!("La Sufficient Condition 2 ha fallat"));

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
